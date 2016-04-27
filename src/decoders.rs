
use color::Color;
use helpers;
use png::PngFile;

pub trait PixelDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color>;
    fn step(&self) -> usize;
}

pub struct OneBitIndexedColorDecoder;
impl PixelDecoder for OneBitIndexedColorDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color> {
        vec![
            png.palette[(val >> 7) as usize].clone(),
            png.palette[(val >> 6) as usize & 0x01].clone(),
            png.palette[(val >> 5) as usize & 0x01].clone(),
            png.palette[(val >> 4) as usize & 0x01].clone(),
            png.palette[(val >> 3) as usize & 0x01].clone(),
            png.palette[(val >> 2) as usize & 0x01].clone(),
            png.palette[(val >> 1) as usize & 0x01].clone(),
            png.palette[val as usize & 0x01].clone()
        ]
    }

    fn step(&self) -> usize {
        0x01
    }
}

pub struct TwoBitIndexedColorDecoder;
impl PixelDecoder for TwoBitIndexedColorDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color> {
        vec![
            png.palette[(val >> 6) as usize].clone(),
            png.palette[(val >> 4) as usize & 0x03].clone(),
            png.palette[(val >> 2) as usize & 0x03].clone(),
            png.palette[val as usize & 0x03].clone()
        ]
    }

    fn step(&self) -> usize {
        0x01
    }
}

pub struct FourBitIndexedColorDecoder;
impl PixelDecoder for FourBitIndexedColorDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color> {
        vec![
            png.palette[(val >> 4) as usize].clone(),
            png.palette[val as usize & 0x0f].clone()
        ]
    }

    fn step(&self) -> usize {
        0x01
    }
}

pub struct EightBitIndexedColorDecoder;
impl PixelDecoder for EightBitIndexedColorDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color> {
        vec![png.palette[val as usize].clone()]
    }

    fn step(&self) -> usize {
        0x01
    }
}

pub struct OneBitGreyscaleDecoder;
impl PixelDecoder for OneBitGreyscaleDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color> {
        let one     = iif!(data[x] >> 7 & 0x01 == 0, 0, 255);
        let two     = iif!(data[x] >> 6 & 0x01 == 0, 0, 255);
        let three   = iif!(data[x] >> 5 & 0x01 == 0, 0, 255);
        let four    = iif!(data[x] >> 4 & 0x01 == 0, 0, 255);
        let five    = iif!(data[x] >> 3 & 0x01 == 0, 0, 255);
        let six     = iif!(data[x] >> 2 & 0x01 == 0, 0, 255);
        let seven   = iif!(data[x] >> 1 & 0x01 == 0, 0, 255);
        let eight   = iif!(data[x] & 0x01 == 0, 0, 255);

        vec![
            Color::new(one, one, one, 255),
            Color::new(two, two, two, 255),
            Color::new(three, three, three, 255),
            Color::new(four, four, four, 255),
            Color::new(five, five, five, 255),
            Color::new(six, six, six, 255),
            Color::new(seven, seven, seven, 255),
            Color::new(eight, eight, eight, 255),
        ]
    }

    fn step(&self) -> usize {
        0x01
    }
}

pub struct TwoBitGreyscaleDecoder;
impl PixelDecoder for TwoBitGreyscaleDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color> {
        let lookup = vec![
            Color::new(0, 0, 0, 255),
            Color::new(85, 85, 85, 255),
            Color::new(170, 170, 170, 255),
            Color::new(255, 255, 255, 255)
        ];

        let one     = (data[x] >> 6 & 0x03) as usize;
        let two     = (data[x] >> 4 & 0x03) as usize;
        let three   = (data[x] >> 2 & 0x03) as usize;
        let four    = (data[x] & 0x03) as usize;

        vec![
            lookup[one].clone(),
            lookup[two].clone(),
            lookup[three].clone(),
            lookup[four].clone()
        ]
    }

    fn step(&self) -> usize {
        0x01
    }
}

pub struct FourBitGreyscaleDecoder;
impl PixelDecoder for FourBitGreyscaleDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color> {
        let lookup = vec![
            Color::new(0, 0, 0, 255),
            Color::new(17, 17, 17, 255),
            Color::new(34, 34, 34, 255),
            Color::new(51, 51, 51, 255),
            Color::new(68, 68, 68, 255),
            Color::new(85, 85, 85, 255),
            Color::new(102, 102, 102, 255),
            Color::new(119, 119, 119, 255),
            Color::new(136, 136, 136, 255),
            Color::new(153, 153, 153, 255),
            Color::new(170, 170, 170, 255),
            Color::new(187, 187, 187, 255),
            Color::new(204, 204, 204, 255),
            Color::new(221, 221, 221, 255),
            Color::new(238, 238, 238, 255),
            Color::new(255, 255, 255, 255),
        ];

        let one     = (data[x] >> 4 & 0x0f) as usize;
        let two     = (data[x] & 0x0f) as usize;

        vec![
            lookup[one].clone(),
            lookup[two].clone()
        ]
    }

    fn step(&self) -> usize {
        0x01
    }
}

pub struct EightBitGreyscaleDecoder;
impl PixelDecoder for EightBitGreyscaleDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color> {
        vec![
            Color::new(
                data[x],
                data[x],
                data[x],
                255
            )
        ]
    }

    fn step(&self) -> usize {
        0x01
    }
}

pub struct SixteenBitGreyscaleDecoder;
impl PixelDecoder for SixteenBitGreyscaleDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color> {
        let combined = ((data[x] as u16) << 8) | data[x + 0x01] as u16;

        let percentage: f64 = combined as f64 / u16::max_value() as f64;
        let result = (percentage * u8::max_value() as f64) as u8;

        vec![
            Color::new(
                result,
                result,
                result,
                255
            )
        ]
    }

    fn step(&self) -> usize {
        0x02
    }
}

pub struct EightBitGreyscaleWithAlphaDecoder;
impl PixelDecoder for EightBitGreyscaleWithAlphaDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color> {
        vec![
            Color::new(
                data[x],
                data[x],
                data[x],
                data[x + 0x01]
            )
        ]
    }

    fn step(&self) -> usize {
        0x02
    }
}

pub struct EightBitTrueColorDecoder;
impl PixelDecoder for EightBitTrueColorDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color> {
        vec![
            Color::new(
                data[x],
                data[x + 0x01],
                data[x + 0x02],
                255
            )
        ]
    }

    fn step(&self) -> usize {
        0x03
    }
}

pub struct SixteenBitTrueColorDecoder;
impl PixelDecoder for SixteenBitTrueColorDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color> {
        let ru16 = ((data[x] as u16) << 8) | data[x + 0x01] as u16;
        let gu16 = ((data[x + 0x02] as u16) << 8) | data[x + 0x03] as u16;
        let bu16 = ((data[x + 0x04] as u16) << 8) | data[x + 0x05] as u16;

        let rp: f64 = ru16 as f64 / u16::max_value() as f64;
        let gp: f64 = gu16 as f64 / u16::max_value() as f64;
        let bp: f64 = bu16 as f64 / u16::max_value() as f64;

        let r = (rp * u8::max_value() as f64) as u8;
        let g = (gp * u8::max_value() as f64) as u8;
        let b = (bp * u8::max_value() as f64) as u8;

        vec![
            Color::new(
                r,
                g,
                b,
                255
            )
        ]
    }

    fn step(&self) -> usize {
        0x06
    }
}

pub struct EightBitTrueColorWithAlphaDecoder;
impl PixelDecoder for EightBitTrueColorWithAlphaDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color> {
        vec![
            Color::new(
                data[x],
                data[x + 0x01],
                data[x + 0x02],
                data[x + 0x03]
            )
        ]
    }

    fn step(&self) -> usize {
        0x04
    }
}

pub struct SixteenBitTrueColorWithAlphaDecoder;
impl PixelDecoder for SixteenBitTrueColorWithAlphaDecoder {
    fn decode(&self, data: &[u8], x: usize, val: u8, png: &PngFile) -> Vec<Color> {
        let ru16 = ((data[x] as u16) << 8) | data[x + 0x01] as u16;
        let gu16 = ((data[x + 0x02] as u16) << 8) | data[x + 0x03] as u16;
        let bu16 = ((data[x + 0x04] as u16) << 8) | data[x + 0x05] as u16;
        let au16 = ((data[x + 0x06] as u16) << 8) | data[x + 0x07] as u16;

        let rp: f64 = ru16 as f64 / u16::max_value() as f64;
        let gp: f64 = gu16 as f64 / u16::max_value() as f64;
        let bp: f64 = bu16 as f64 / u16::max_value() as f64;
        let ap: f64 = au16 as f64 / u16::max_value() as f64;

        let r = (rp * u8::max_value() as f64) as u8;
        let g = (gp * u8::max_value() as f64) as u8;
        let b = (bp * u8::max_value() as f64) as u8;
        let a = (ap * u8::max_value() as f64) as u8;

        vec![
            Color::new(
                r,
                g,
                b,
                a
            )
        ]
    }

    fn step(&self) -> usize {
        0x08
    }
}
