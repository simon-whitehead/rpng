
use color::Color;
use png::PngFile;

pub trait PixelDecoder {
    fn decode(&self, data: &[u8], png: &PngFile) -> Vec<Color>;
}

pub struct FourBitIndexedColorDecoder;
impl PixelDecoder for FourBitIndexedColorDecoder {
    fn decode(&self, data: &[u8], png: &PngFile) -> Vec<Color> {
        let mut pixels = Vec::new();
        let mut lookup = Vec::new();

        for y in 0..png.h {
            let mut x = 0;
            let row_start = y * (png.pitch + 1);
            let pixel_start = row_start + 1;
            while x < png.pitch {
                let mut val = data[pixel_start + x] as u8;
                let left = val >> 4;
                let right = val & 0x0f;
            
                lookup.push(left);
                lookup.push(right);

                x += 0x01; // 1 byte
            }
        }

        for i in 0..lookup.len() {
            let pixel = png.palette[lookup[i] as usize].clone();
            pixels.push(pixel);
        }

        pixels
    }
}

pub struct EightBitTrueColorWithAlphaDecoder;
impl PixelDecoder for EightBitTrueColorWithAlphaDecoder {
    fn decode(&self, data: &[u8], png: &PngFile) -> Vec<Color> {
        let mut pixels = Vec::new();

        for y in 0..png.h {
            let mut i = 0;
            let row_start = y * (png.pitch + 1);
            let pixel_start = row_start + 1;
            while i < png.pitch {
                let x = pixel_start + i;
                pixels.push(
                    Color::new(
                        data[x],
                        data[x + 0x01],
                        data[x + 0x02],
                        data[x + 0x03]
                    )
                );
                i += 0x04; // 4 bytes, 8 bits per sample
            }
        }

        pixels
    }
}
