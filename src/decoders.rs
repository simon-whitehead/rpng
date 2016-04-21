
use color::Color;
use png::PngFile;

pub trait PixelDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color>;
    fn step(&self) -> usize;
}

pub struct OneBitIndexedColorDecoder;
impl PixelDecoder for OneBitIndexedColorDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color> {
        let mut pixels = Vec::new();

        pixels.push(png.palette[(val >> 7) as usize].clone());
        pixels.push(png.palette[(val >> 6) as usize & 0x01].clone());
        pixels.push(png.palette[(val >> 5) as usize & 0x01].clone());
        pixels.push(png.palette[(val >> 4) as usize & 0x01].clone());
        pixels.push(png.palette[(val >> 3) as usize & 0x01].clone());
        pixels.push(png.palette[(val >> 2) as usize & 0x01].clone());
        pixels.push(png.palette[(val >> 1) as usize & 0x01].clone());
        pixels.push(png.palette[val as usize & 0x01].clone());

        pixels
    }

    fn step(&self) -> usize {
        0x01
    }
}

pub struct TwoBitIndexedColorDecoder;
impl PixelDecoder for TwoBitIndexedColorDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color> {
        let mut pixels = Vec::new();

        pixels.push(png.palette[(val >> 6) as usize].clone());
        pixels.push(png.palette[(val >> 4) as usize & 0x03].clone());
        pixels.push(png.palette[(val >> 2) as usize & 0x03].clone());
        pixels.push(png.palette[val as usize & 0x03].clone());

        pixels
    }

    fn step(&self) -> usize {
        0x01
    }
}

pub struct FourBitIndexedColorDecoder;
impl PixelDecoder for FourBitIndexedColorDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color> {
        let mut pixels = Vec::new();
        pixels.push(png.palette[(val >> 4) as usize].clone());
        pixels.push(png.palette[val as usize & 0x0f].clone());

        pixels
    }

    fn step(&self) -> usize {
        0x01
    }
}

pub struct EightBitIndexedColorDecoder;
impl PixelDecoder for EightBitIndexedColorDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color> {
        let mut pixels = Vec::new();
        pixels.push(png.palette[val as usize].clone());

        pixels
    }

    fn step(&self) -> usize {
        0x01
    }
}

pub struct EightBitTrueColorWithAlphaDecoder;
impl PixelDecoder for EightBitTrueColorWithAlphaDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color> {
        let mut pixels = Vec::new();

        pixels.push(
            Color::new(
                data[x],
                data[x + 0x01],
                data[x + 0x02],
                data[x + 0x03]
            )
        );

        pixels
    }

    fn step(&self) -> usize {
        0x04
    }
}
