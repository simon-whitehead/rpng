use std::fs::File;
use std::io::Read;
use std::ops::Deref;

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
    w: usize,
    h: usize,

    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,

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

            png.read_chunks(rest)
        } else {
            Err(PngError::InvalidHeader)
        }
    }

    #[inline(always)]
    fn advance(&mut self, distance: usize) {
        self.idx += distance;
    }

    pub fn read_chunks(mut self, data: &[u8]) -> PngLoadResult {
        // Grab length of chunk
        let length = helpers::read_unsigned_int(data);
        self.advance(4);

        // The ImageHeader (IHDR) chunk should be first
        let ihdr = &data[self.idx..self.idx+4];
        if ihdr == b"IHDR" {
            println!("Found IHDR chunk");

            // Parse the IHDR chunk
            if let Err(error) = self.parse_ihdr(data) {
                return Err(PngError::InvalidFormat(error));
            }

            // We found an IHDR chunk... now lets just loop over every chunk we find and 
            // work with it
            println!("Width: {}px, Height: {}px", self.w, self.h);
        } else {
            return Err(PngError::InvalidFormat("IHDR chunk missing".to_string()))
        }

        Ok(self)
    }

    fn parse_ihdr(&mut self, data: &[u8]) -> PngParseResult {
        self.advance(4);

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
