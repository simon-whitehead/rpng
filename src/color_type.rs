use std::fmt;

#[derive(PartialEq)]
pub enum ColorType {
    Unknown,
    Greyscale,
    Truecolor,
    IndexedColor,
    GreyscaleWithAlpha,
    TrueColorWithAlpha
}

impl ColorType {
    /// Asserts that the ColorType and Bit Depth combination is
    /// valid according to the PNG spec.
    pub fn validate(&self, bit_depth: u8) -> Result<(), String> {
        if *self != ColorType::Greyscale &&
           *self != ColorType::Truecolor &&
           *self != ColorType::IndexedColor &&
           *self != ColorType::GreyscaleWithAlpha &&
           *self != ColorType::TrueColorWithAlpha {
            
               return Err(format!("Invalid colour type, found: {}", *self));
        }

        let color_type_bit_depth_err = "Invalid color type and bit depth combination".to_string();

        if *self == ColorType::Greyscale && (
            bit_depth != 1 &&
            bit_depth != 2 &&
            bit_depth != 4 &&
            bit_depth != 8 &&
            bit_depth != 16
        ) {
            return Err(color_type_bit_depth_err);
        }

        if *self == ColorType::Truecolor && (
            bit_depth != 8 &&
            bit_depth != 16
        ) {
            return Err(color_type_bit_depth_err);
        }

        if *self == ColorType::IndexedColor && (
            bit_depth != 1 &&
            bit_depth != 2 &&
            bit_depth != 4 &&
            bit_depth != 8
        ) {
            return Err(color_type_bit_depth_err);
        }

        if *self == ColorType::GreyscaleWithAlpha && (
            bit_depth != 8 &&
            bit_depth != 16
        ) {
            return Err(color_type_bit_depth_err);
        }

        if *self == ColorType::TrueColorWithAlpha && (
            bit_depth != 8 &&
            bit_depth != 16
        ) {
            return Err(color_type_bit_depth_err);
        }

        Ok(())
    }
}

impl fmt::Display for ColorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ColorType::Unknown => write!(f, "Unknown"),
            ColorType::Greyscale => write!(f, "Greyscale"),
            ColorType::Truecolor => write!(f, "Truecolor"),
            ColorType::IndexedColor => write!(f, "Indexed-color"),
            ColorType::GreyscaleWithAlpha => write!(f, "Greyscale with alpha"),
            ColorType::TrueColorWithAlpha => write!(f, "Truecolor with alpha")
        }
    }
}

impl From<u8> for ColorType {
    fn from(b: u8) -> Self {
        match b {
            0 => ColorType::Greyscale,
            2 => ColorType::Truecolor,
            3 => ColorType::IndexedColor,
            4 => ColorType::GreyscaleWithAlpha,
            6 => ColorType::TrueColorWithAlpha,
            _ => ColorType::Unknown
        }
    }
}
