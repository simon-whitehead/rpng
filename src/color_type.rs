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
