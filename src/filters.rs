
use png::PngFile;

pub trait Filter {
    fn apply(&self, x: u8, a: u8, b: u8, c: u8) -> u8;
}

pub struct NoFilter;
impl Filter for NoFilter {
    fn apply(&self, x: u8, a: u8, b: u8, c: u8) -> u8 {
        // No op
        x
    }
}

pub struct Sub;
impl Filter for Sub {
    fn apply(&self, x: u8, a: u8, b: u8, c: u8) -> u8 {
        (x as u32 + a as u32) as u8
    }
}

pub struct Up;
impl Filter for Up {
    fn apply(&self, x: u8, a: u8, b: u8, c: u8) -> u8 {
        (x as u32 + b as u32) as u8
    }
}

pub struct Average;
impl Filter for Average {
    fn apply(&self, x: u8, a: u8, b: u8, c: u8) -> u8 {
        (x as u32 + ((a as u32 + b as u32) / 2)) as u8
    }
}

pub struct Paeth;
impl Filter for Paeth {
    fn apply(&self, x: u8, a: u8, b: u8, c: u8) -> u8 {
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

