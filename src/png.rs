extern crate flate2;

use std::fs::File;
use std::io::Read;
use std::ops::Deref;

use self::flate2::read::ZlibDecoder;

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
    InvalidHeader,
    InvalidFormat(String)
}

pub struct ScanLine {
    pub filter_type: u8,
    pub pixels: Vec<Color>
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

pub enum PngChunkType {
    // Critical
    ImageHeader,
    Palette,
    ImageData,
    End,

    // Ancillary
    Chromaticity,
    Gamma,
    ICCProfile,
    SignificantBits,
    RGBColorSpace,
    BackgroundColor,
    Histogram,
    Transparency,
    PhysicalPixelDimensions,
    SuggestedPalette,
    LastModifiedTime,
    InternationalTextualData,
    TextualData,
    CompressedTextualData
}

pub enum PngImageType {
    Greyscale,
    Truecolor,
    IndexedColor,
    GreyscaleWithAlpha,
    TrueColorWithAlpha
}

pub struct PngFile {
    pub w: usize,
    pub h: usize,

    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,

    image_data_chunks: Vec<Vec<u8>>,

    pub pitch: usize,
    pub scan_lines: Vec<ScanLine>,

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
            color_type: 0,
            compression_method: 0,
            filter_method: 0,
            interlace_method: 0,

            image_data_chunks: Vec::new(),

            pitch: 0,
            scan_lines: Vec::new(),

            significant_bits: [0; 4],

            idx: 0,

            found_ihdr: false
        }
    }

    pub fn default() -> Self {
        Self::new(0, 0)
    }

    /// Load PNG from given path
    pub fn from_path(path: &str) -> PngLoadResult {
        let mut data: Vec<u8> = Vec::new();
        if let Ok(mut file) = File::open(path) {
            let _ = file.read_to_end(&mut data); 
        }
        Self::from_data(&data)
    }

    pub fn from_data(file_data: &[u8]) -> PngLoadResult {
        let mut png = Self::default();

        let header = &file_data[0..8];
        if header == PNG_HEADER {
            let rest = &file_data[8..];

            if let Err(message) = png.read_chunks(rest) {
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
            println!("DEFLATE stream size: {}", compressed_data.len());
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
                let row = &decompressed_data[y * row_size..y * row_size + row_size];
                println!("Row filter byte: {}", &row[0]);
                let p = &row[1..];
                self.pitch = p.len();
                let mut i = 0;
                let mut pixels = Vec::new();
                while i < p.len(){
                    pixels.push(Color::new(p[i], p[i+1], p[i+2], p[i+3]));
                    i+=4;
                }
                let scan_line = ScanLine {
                    filter_type: row[0],
                    pixels: pixels
                };
                self.scan_lines.push(scan_line);
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
            println!("Width: {}px, Height: {}px", self.w, self.h);

            loop {
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
        if self.color_type == 0 {
            self.significant_bits[0] = data[0];
        } else if self.color_type == 2 || self.color_type == 3 {
            self.significant_bits[0] = data[0];
            self.significant_bits[1] = data[1];
            self.significant_bits[2] = data[2];
        } else if self.color_type == 4 {
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
        self.color_type = data[self.idx];
        self.advance(1);
        self.compression_method = data[self.idx];
        self.advance(1);
        self.filter_method = data[self.idx];
        self.advance(1);
        self.interlace_method = data[self.idx];
        self.advance(1);

        println!("Color type: {}, Bit depth: {}, Interlace method: {}, Filter method: {}", self.color_type, self.bit_depth, self.interlace_method, self.filter_method);

        // Skip the CRC
        self.advance(4);

        if let Err(message) = self.check_color_types_and_values() {
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

    fn check_color_types_and_values(&self) -> PngParseResult {
        if self.color_type != 0 &&
           self.color_type != 2 &&
           self.color_type != 3 &&
           self.color_type != 4 &&
           self.color_type != 6 {
            
               return Err(format!("Invalid colour type, found: {}", self.color_type));
        }

        let color_type_bit_depth_err = "Invalid color type and bit depth combination".to_string();

        if self.color_type == 0 && (
            self.bit_depth != 1 &&
            self.bit_depth != 2 &&
            self.bit_depth != 4 &&
            self.bit_depth != 8 &&
            self.bit_depth != 16
        ) {
            return Err(color_type_bit_depth_err);
        }

        if self.color_type == 2 && (
            self.bit_depth != 8 &&
            self.bit_depth != 16
        ) {
            return Err(color_type_bit_depth_err);
        }

        if self.color_type == 3 && (
            self.bit_depth != 1 &&
            self.bit_depth != 2 &&
            self.bit_depth != 4 &&
            self.bit_depth != 8
        ) {
            return Err(color_type_bit_depth_err);
        }

        if self.color_type == 4 && (
            self.bit_depth != 8 &&
            self.bit_depth != 16
        ) {
            return Err(color_type_bit_depth_err);
        }

        if self.color_type == 6 && (
            self.bit_depth != 8 &&
            self.bit_depth != 16
        ) {
            return Err(color_type_bit_depth_err);
        }

        Ok(())
    }
}
