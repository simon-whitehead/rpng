mod color;
mod color_type;
mod deflate;
mod error;
mod helpers;
mod ihdr;
mod png;

pub use self::png::PngFile;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
