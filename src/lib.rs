#[macro_use]
mod helpers;

mod color;
mod color_type;
mod decoders;
mod deflate;
mod error;
mod filters;
mod ihdr;
mod png;

pub use self::png::PngFile;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
