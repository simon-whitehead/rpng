
use png::PngFile;

pub trait Filter {
    fn apply(&self, x: u16, a: u16, b: u16, c: u16) -> u8;
}

pub struct NoFilter;
impl Filter for NoFilter {
    fn apply(&self, x: u16, a: u16, b: u16, c: u16) -> u8 {
        // No op
        x as u8
    }
}

pub struct Sub;
impl Filter for Sub {
    fn apply(&self, x: u16, a: u16, b: u16, c: u16) -> u8 {
        (x + a) as u8
    }
}

pub struct Up;
impl Filter for Up {
    fn apply(&self, x: u16, a: u16, b: u16, c: u16) -> u8 {
        (x + b) as u8
    }
}

pub struct Average;
impl Filter for Average {
    fn apply(&self, x: u16, a: u16, b: u16, c: u16) -> u8 {
        (x + ((a + b) / 2)) as u8
    }
}

pub struct Paeth;
impl Filter for Paeth {
    fn apply(&self, x: u16, a: u16, b: u16, c: u16) -> u8 {
        let (a, b, c) = (a as i32, b as i32, c as i32);
        let p: i32 = a + b - c;
        let pa = (p - a).abs();
        let pb = (p - b).abs();
        let pc = (p - c).abs();
        if pa <= pb && pa <= pc {
            ((x as i32 + a as i32) % 256) as u8
        } else if pb <= pc {
            ((x as i32 + b as i32) % 256) as u8
        } else {
            ((x as i32 + c as i32) % 256) as u8
        }

    }
}

