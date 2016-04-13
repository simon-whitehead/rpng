extern crate flate2;

use std::fs::File;
use std::io::{Error, Read};
use std::ops::Deref;
use std::path::Path;

use self::flate2::read::ZlibDecoder;

use color_type::ColorType;
use helpers;

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

#[derive(Debug)]
pub enum PngError {
    Io(Error),
    InvalidHeader,
    InvalidFormat(String)
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            r: r,
            g: g,
            b: b,
            a: a
        }
    }
}

pub struct PngFile {
    pub w: usize,
    pub h: usize,

    bit_depth: u8,
    color_type: ColorType,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,

    image_data_chunks: Vec<Vec<u8>>,

    pub pitch: usize,
    pub pixels: Vec<Color>,

    // sBIT
    significant_bits: [u8; 4],

    idx: usize,

    found_ihdr: bool
}

impl PngFile {
    pub fn new(width: usize, height: usize) -> Self {
        PngFile {
            w: 0,
            h: 0,

            bit_depth: 0,
            color_type: ColorType::Unknown,
            compression_method: 0,
            filter_method: 0,
            interlace_method: 0,

            image_data_chunks: Vec::new(),

            pitch: 0,
            pixels: Vec::new(),

            significant_bits: [0; 4],

            idx: 0,

            found_ihdr: false
        }
    }

    pub fn default() -> Self {
        Self::new(0, 0)
    }

    /// Load PNG from given path
    pub fn from_path<P: AsRef<Path>>(path: P) -> PngLoadResult {
        let mut data: Vec<u8> = Vec::new();
        match File::open(path) {
            Ok(mut file) => file.read_to_end(&mut data),
            Err(err) => return Err(PngError::Io(err))
        };
        Self::from_data(&data)
    }

    pub fn from_data(file_data: &[u8]) -> PngLoadResult {
        let mut png = Self::default();

        // Check that we have what looks like a
        // PNG file.
        let header = &file_data[0..8];
        if header == PNG_HEADER {
            if let Err(message) = png.read_chunks(&file_data[8..]) {
                Err(PngError::InvalidFormat(message))
            } else {
                if let Err(message) = png.decode_pixel_data() {
                    Err(PngError::InvalidFormat(message))
                } else {
                    Ok(png)
                }
            }
        } else {
            Err(PngError::InvalidHeader)
        }
    }

    #[inline(always)]
    fn advance(&mut self, distance: usize) {
        self.idx += distance;
    }

    fn decode_pixel_data(&mut self) -> PngParseResult {
        let mut compressed_data = Vec::new();

        for chunk in &mut self.image_data_chunks {
            compressed_data.append(chunk);
        }
            let predict = (((self.w / 8) * 32) + ((self.w & 7) * 32 + 7) / 8) * self.h;
            let mut decompressed_data = Vec::new();
            let mut buf = Vec::with_capacity(predict);
            let mut decompressor = ZlibDecoder::new(&compressed_data[..]);
            match decompressor.read_to_end(&mut buf) {
                Ok(n) => {
                    if n != 0 {
                        decompressed_data.extend(buf.iter().cloned());
                    }
                },
                Err(err) => return Err(err.to_string())
            }
            let row_size = (1 + ((self.bit_depth * 4) as usize*self.w+7)/8) as usize;
            for y in 0..self.h {
                self.pitch = row_size - 1;
                let mut i = 0;
                let row_start = y * row_size;
                let filter_type = decompressed_data[row_start];
                let pixel_start = row_start + 1;
                // Apply the filters
                while i < row_size - 1 {
                    let x = pixel_start + i;
                    if filter_type == 1 {
                        if x - pixel_start > 3 {
                            let result = decompressed_data[x] as u32 + decompressed_data[x-4] as u32;
                            decompressed_data[x] = result as u8;
                        }
                    } else if filter_type == 2 {
                        if y > 0 {
                            let prev_x = x - row_size;
                            let pixel_above = decompressed_data[prev_x];
                            let pixel = decompressed_data[x];

                            let result = pixel as u32 + pixel_above as u32;

                            decompressed_data[x] = result as u8;
                        }
                    } else if filter_type == 3 {
                        let prev_x = x - row_size;
                        let pixel_above = decompressed_data[prev_x];
                        let pixel = decompressed_data[x];
                        if x - pixel_start > 3 && y > 0 {
                            let west_pixel = decompressed_data[x-4];
                            let result = pixel as u32 + ((west_pixel as u32 + pixel_above as u32) / 2) as u32;
                            decompressed_data[x] = result as u8;
                        } else {
                            let result = (pixel as u32 + pixel_above as u32) / 2;
                            decompressed_data[x] = result as u8;
                        }
                    } else if filter_type == 4 {
                        // Paeth
                        if x - pixel_start > 3 && y > 0 {
                            let prev_x = x - row_size;
                            let prev_prev_x = prev_x - 4;
                            let upper_left = decompressed_data[prev_prev_x] as i32;
                            let above = decompressed_data[prev_x] as i32;
                            let left = decompressed_data[x - 4] as i32;

                            let p: i32 = left + above - upper_left;
                            let pa = (p - left).abs();
                            let pb = (p - above).abs();
                            let pc = (p - upper_left).abs();
                            if pa <= pb && pa <= pc {
                                decompressed_data[x] = (decompressed_data[x] as i32 + left as i32) as u8;
                            } else if pb <= pc {
                                decompressed_data[x] = (decompressed_data[x] as i32 + above as i32) as u8;
                            } else {
                                decompressed_data[x] = (decompressed_data[x] as i32 + upper_left as i32) as u8;
                            }
                        }
                    }
                    i+=1;
                }
            }

            for y in 0..self.h {
                self.pitch = row_size - 1;
                let mut i = 0;
                let mut pixels = Vec::new();
                let row_start = y * row_size;
                let filter_type = decompressed_data[row_start];
                let pixel_start = row_start + 1;
                while i < row_size - 1 {
                    let x = pixel_start + i;
                    pixels.push(Color::new(decompressed_data[x], decompressed_data[x + 1], decompressed_data[x + 2], decompressed_data[x + 3]));
                    i+=4;
                }

                self.pixels.extend(pixels);
            }

        Ok(())
    }

    pub fn read_chunks(&mut self, data: &[u8]) -> PngParseResult {
        // Grab length of chunk
        let length = helpers::read_unsigned_int(data);
        self.advance(4);

        // The ImageHeader (IHDR) chunk should be first
        let ihdr = &data[self.idx..self.idx+4];
        if ihdr == b"IHDR" {
            // Parse the IHDR chunk
            self.advance(4);

            if let Err(error) = self.parse_ihdr(data) {
                return Err(error);
            }

            // We found an IHDR chunk... now lets just loop over every chunk we find and 
            // work with it
            loop {
                // Read the chunk length, type and its data
                let chunk_length = helpers::read_unsigned_int(&data[self.idx..]) as usize;
                self.advance(4);
                let chunk_type = &data[self.idx..self.idx+4];
                self.advance(4);
                let chunk_data = &data[self.idx..self.idx+chunk_length];

                match chunk_type {
                    b"IDAT" => self.image_data_chunks.push(chunk_data.iter().cloned().collect()),
                    b"sBIT" => self.parse_sbit(chunk_data),
                    b"IEND" => { println!("Found end!"); break; },
                    b"PLTE" => println!("found palette chunk"),
                    n => println!("Found chunk: {}", String::from_utf8(n.iter().cloned().collect()).unwrap())
                };

                self.advance(chunk_data.len() + 4); // The chunk plus the CRC
            }
        } else {
            return Err("IHDR chunk missing".to_string())
        }

        Ok(())
    }

    fn parse_sbit(&mut self, data: &[u8]) {
        if self.color_type == ColorType::Greyscale {
            self.significant_bits[0] = data[0];
        } else if self.color_type == ColorType::Truecolor || self.color_type == ColorType::IndexedColor {
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

    fn parse_ihdr(&mut self, data: &[u8]) -> PngParseResult {
        // Store the width and height
        self.w = helpers::read_unsigned_int(&data[self.idx..]) as usize;
        self.advance(4);
        self.h = helpers::read_unsigned_int(&data[self.idx..]) as usize;
        self.advance(4);

        // Store the rest of the IHDR metadata
        self.bit_depth = data[self.idx];
        self.advance(1);
        self.color_type = ColorType::from(data[self.idx]);
        self.advance(1);
        self.compression_method = data[self.idx];
        self.advance(1);
        self.filter_method = data[self.idx];
        self.advance(1);
        self.interlace_method = data[self.idx];
        self.advance(1);

        // Skip the CRC
        self.advance(4);

        if let Err(message) = self.color_type.validate(self.bit_depth) {
            return Err(message);
        }

        if self.compression_method != 0 {
            return Err("Compression method invalid".to_string());
        }

        if self.filter_method != 0 {
            return Err("Filter method invalid".to_string());
        }

        if self.interlace_method > 1 {
            return Err("Interlace method invalid".to_string());
        }

        Ok(())
    }
}
