
use std::fs::File;
use std::io::Read;
use std::path::Path;

use color::Color;
use color_type::ColorType;
use decoders::{PixelDecoder, FourBitIndexedColorDecoder, EightBitTrueColorWithAlphaDecoder};
use deflate;
use error::PngError;
use filters::{Filter, NoFilter, Sub, Up, Average, Paeth};
use helpers;
use ihdr;

const PNG_HEADER: [u8; 8] = [
    0x89,
    'P' as u8,
    'N' as u8,
    'G' as u8,
    0x0D,
    0x0A,
    0x1A,
    0x0A
];

pub type PngLoadResult = Result<PngFile, PngError>;
pub type PngParseResult = Result<(), String>;

pub struct PngFile {
    pub w: usize,
    pub h: usize,

    bit_depth: usize,
    pub bits_per_pixel: usize,
    pub bytes_per_pixel: usize,
    color_type: ColorType,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,

    image_data_chunks: Vec<Vec<u8>>,

    pub pitch: usize,
    pub pixels: Vec<Color>,

    pub palette: Vec<Color>,

    // sBIT
    significant_bits: [u8; 4],

    idx: usize
}

impl PngFile {
    pub fn new(width: usize, height: usize) -> Self {
        PngFile {
            w: width,
            h: height,

            bit_depth: 0,
            bits_per_pixel: 0,
            bytes_per_pixel: 0,
            color_type: ColorType::Unknown,
            compression_method: 0,
            filter_method: 0,
            interlace_method: 0,

            image_data_chunks: Vec::new(),

            pitch: 0,
            pixels: Vec::new(),

            palette: Vec::new(),

            significant_bits: [0; 4],

            idx: 0
        }
    }

    pub fn default() -> Self {
        Self::new(0, 0)
    }

    /// Loads a PNG from given path.
    pub fn from_path<P: AsRef<Path>>(path: P) -> PngLoadResult {
        let mut data: Vec<u8> = Vec::new();
        match File::open(path) {
            Ok(mut file) => try!(file.read_to_end(&mut data)),
            Err(err) => return Err(PngError::Io(err))
        };
        Self::from_data(&data)
    }

    /// Parses a byte slice as a PNG file.
    pub fn from_data(file_data: &[u8]) -> PngLoadResult {
        let mut png = Self::default();

        // Check that we have what looks like a
        // PNG file.
        let header = &file_data[0..8];
        if header != PNG_HEADER {
            Err(PngError::InvalidHeader)
        } else {
            try!(png.read_chunks(&file_data[0x08..]));
            try!(png.decode_pixel_data());

            Ok(png)
        }
    }

    #[inline(always)]
    fn advance(&mut self, distance: usize) {
        self.idx += distance;
    }

    pub fn read_chunks(&mut self, data: &[u8]) -> PngParseResult {
        self.advance(4); // Jump over the IHDR u32 length bytes

        // The ImageHeader (IHDR) chunk should be first
        let ihdr = &data[self.idx..self.idx+4];
        if ihdr == b"IHDR" {
            // Parse the IHDR chunk
            try!(self.parse_ihdr(&data[..]));
            self.calculate_bpp();

            // We found an IHDR chunk... now lets just loop over every chunk we find and 
            // work with it
            loop {
                // Read the chunk length, type and its data
                let chunk_length = helpers::read_unsigned_int(&data[self.idx..]) as usize;
                let chunk_type = &data[self.idx + 0x04..self.idx + 0x08];
                let chunk_data = &data[self.idx + 0x08..self.idx + chunk_length + 0x08];

                match chunk_type {
                    b"IDAT" => self.image_data_chunks.push(chunk_data.iter().cloned().collect()),
                    b"PLTE" => {
                        if chunk_length % 3 == 0 {
                            self.build_palette(&chunk_data);
                        } else {
                            return Err("Invalid palette length".to_string());
                        }
                    },
                    b"sBIT" => self.parse_sbit(&chunk_data),
                    b"IEND" => { break; },
                    n => /*println!("Found chunk: {}", String::from_utf8(n.iter().cloned().collect()).unwrap())*/()
                };

                self.advance(chunk_data.len() + 0x0C); // The chunk length, type, data and CRC
            }
        } else {
            return Err("IHDR chunk missing".to_string())
        }

        Ok(())
    }

    fn parse_ihdr(&mut self, data: &[u8]) -> PngParseResult {
        match ihdr::parse(&data[self.idx..]) {
            Err(error) => return Err(error),
            Ok(ihdr) => {
                self.w = ihdr.width;
                self.h = ihdr.height;
                self.bit_depth = ihdr.bit_depth as usize;
                self.color_type = ihdr.color_type;
                self.compression_method = ihdr.compression_method;
                self.filter_method = ihdr.filter_method;
                self.interlace_method = ihdr.interlace_method;
            }
        };

        self.advance(0x15); // The IHDR chunk type, data and CRC

        Ok(())
    }

    /// Decides how many bits and bytes per pixel there are for this
    /// image based on the ColorType
    fn calculate_bpp(&mut self) {
        match self.color_type {
            ColorType::Unknown => {
                self.bits_per_pixel = 0;
            },
            ColorType::Greyscale => {
                self.bits_per_pixel = self.bit_depth;
            },
            ColorType::TrueColor => {
                self.bits_per_pixel = 24;
            },
            ColorType::IndexedColor => {
                self.bits_per_pixel = self.bit_depth;
            },
            ColorType::GreyscaleWithAlpha => {
                self.bits_per_pixel = 16;
            },
            ColorType::TrueColorWithAlpha => {
                self.bits_per_pixel = 32;
            } 
        }

        self.bytes_per_pixel = (self.bits_per_pixel + 7) / 8;
    }

    fn decode_pixel_data(&mut self) -> PngParseResult {
        let mut pixels = try!(self.get_pixel_data());
        let row_size = 1 + (self.bits_per_pixel * self.w + 7) / 8;
        self.pitch = row_size - 1;

        self.apply_filters(&mut pixels, row_size);
        self.pixels = self.build_pixels(&mut pixels, row_size);

        /*match self.color_type {
            ColorType::IndexedColor => {
                let mut lookup = Vec::new();
                for y in 0..self.h {
                    let row_start = y * row_size;
                    let filter_type = pixels[row_start];
                    let pixel_start = row_start + 1;
                    let mut x = 0;
                    match self.bit_depth {
                        4 => {
                            while x < row_size - 1 {
                                let mut val = pixels[pixel_start + x] as u8;
                                let left = val >> 4;
                                let right = val & 0x0f;
                            
                                lookup.push(left);
                                lookup.push(right);

                                x += 1;
                            }
                        },
                        _ => unimplemented!()
                    }
                }

                for i in 0..lookup.len() {
                    let pixel = self.palette[lookup[i] as usize].clone();
                    self.pixels.push(pixel);
                }
            },
            ColorType::TrueColorWithAlpha => {

                for y in 0..self.h {
                    let mut i = 0;
                    let mut result = Vec::new();
                    let row_start = y * row_size;
                    let pixel_start = row_start + 1;
                    while i < row_size - 1 {
                        let x = pixel_start + i;
                        match self.bytes_per_pixel {
                            4 => result.push(Color::new(pixels[x], pixels[x + 1], pixels[x + 2], pixels[x + 3])),
                            3 => result.push(Color::new(pixels[x], pixels[x + 1], pixels[x + 2], 255)),
                            2 => result.push(Color::new(pixels[x], pixels[x], pixels[x], 255)),
                            1 => result.push(Color::new(pixels[x], pixels[x], pixels[x], pixels[x])), // LOOKUP PALLETTE
                            _ => ()
                        }
                        
                        i += self.bytes_per_pixel;
                    }

                    self.pixels.extend(result);
                }
            },
            _ => unimplemented!()
        };*/


        Ok(())
    }

    /// Applies scanline filtering depending on the filter type
    /// that is specified at the start of each row.
    fn apply_filters(&self, pixels: &mut [u8], row_size: usize) {
        for y in 0..self.h {
            let mut i = 0;
            let row_start = y * row_size;
            let filter_type = pixels[row_start];
            let pixel_start = row_start + 1;
            let filter: Box<Filter> = 
                match filter_type {
                    0 => Box::new(NoFilter),
                    1 => Box::new(Sub),
                    2 if y > 0 => Box::new(Up),
                    3 if y > 0 => Box::new(Average),
                    4 if y > 0 => Box::new(Paeth),
                    _ => unreachable!()
                };

            filter.apply(
                &mut pixels[..], 
                pixel_start,
                &self
            );
        }
    }

    fn build_pixels(&self, pixels: &mut [u8], row_size: usize) -> Vec<Color> {
        let decoder: Box<PixelDecoder> = 
            match (&self.color_type, self.bit_depth) {
                (&ColorType::IndexedColor, 4) => Box::new(FourBitIndexedColorDecoder),
                (&ColorType::TrueColorWithAlpha, 8) => Box::new(EightBitTrueColorWithAlphaDecoder),
                _ => unreachable!()
            };

        decoder.decode(
            &mut pixels[..],
            &self
        )
    }

    fn get_pixel_data(&mut self) -> Result<Vec<u8>, String> {
        let mut compressed_data = Vec::new();

        for chunk in &mut self.image_data_chunks {
            compressed_data.append(chunk);
        }

        let prediction = (((self.w / 8) * self.bits_per_pixel) + ((self.w & 7) * self.bits_per_pixel + 7) / 8) * self.h;
        deflate::decode(&compressed_data[..], || prediction)
    }

    fn build_palette(&mut self, data: &[u8]) {
       let mut i = 0;
       while i < data.len() {
           let pixel = Color::new(data[i], data[i + 1], data[i + 2], 255);
           self.palette.push(pixel);

           i += 3;
       }
    }

    fn parse_sbit(&mut self, data: &[u8]) {
        if self.color_type == ColorType::Greyscale {
            self.significant_bits[0] = data[0];
        } else if self.color_type == ColorType::TrueColor || self.color_type == ColorType::IndexedColor {
            self.significant_bits[0] = data[0];
            self.significant_bits[1] = data[1];
            self.significant_bits[2] = data[2];
        } else if self.color_type == ColorType::GreyscaleWithAlpha {
            self.significant_bits[0] = data[0];
            self.significant_bits[1] = data[1];
        } else {
            self.significant_bits[0] = data[0];
            self.significant_bits[1] = data[1];
            self.significant_bits[2] = data[2];
            self.significant_bits[3] = data[3];
        }
    }
}
