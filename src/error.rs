use std::io::Error;

#[derive(Debug)]
pub enum PngError {
    Io(Error),
    InvalidHeader,
    InvalidFormat(String)
}

impl From<Error> for PngError {
    fn from(err: Error) -> Self {
        PngError::Io(err)
    }
}
