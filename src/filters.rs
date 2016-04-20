
use png::PngFile;

pub trait Filter {
    fn apply(&self, data: &mut [u8], start: usize, row: usize, png: &PngFile);
}

pub struct NoFilter;
impl Filter for NoFilter {
    fn apply(&self, _data: &mut [u8], _start: usize, row: usize, _png: &PngFile) {
        // No op
    }
}

pub struct Sub;
impl Filter for Sub {
    fn apply(&self, data: &mut [u8], start: usize, row: usize, png: &PngFile) {
        let mut i = 0;
        while i < png.pitch {
            let x = start + i;
            if x - start > png.bytes_per_pixel - 1 {
                let result = data[x] as u32 + data[x - png.bytes_per_pixel] as u32;
                data[x] = result as u8;
            }

            i += 1;
        }
    }
}

pub struct Up;
impl Filter for Up {
    fn apply(&self, data: &mut [u8], start: usize, row: usize, png: &PngFile) {
        let mut i = 0;
        while i < png.pitch {
            let x = start + i;
            let prev_x = x - (png.pitch + 1); // +1 for the filter type on the row
            let pixel_above = if row > 0 { data[prev_x] } else { 0 };
            let pixel = data[x];

            data[x] = (pixel as u16 + pixel_above as u16) as u8;

            i += 1;
        }
    }
}

pub struct Average;
impl Filter for Average {
    fn apply(&self, data: &mut [u8], start: usize, row: usize, png: &PngFile) {
        let mut i = 0;
        while i < png.pitch {
            let x = start + i;
            let prev_x = x - (png.pitch + 1);
            let pixel_above = if row > 0 { data[prev_x] } else { 0 };
            let pixel = data[x];
            let west_pixel = if x - start > png.bytes_per_pixel - 1 { data[x - png.bytes_per_pixel] } else { 0 };
            let result = pixel as u32 + ((west_pixel as u32 + pixel_above as u32) / 2) as u32;
            data[x] = result as u8;

            i += 1;
        }
    }
}

pub struct Paeth;
impl Filter for Paeth {
    fn apply(&self, data: &mut [u8], start: usize, row: usize, png: &PngFile) {
        let mut i = 0;
        while i < png.pitch {
            let x = start + i;
            let prev_x = x - (png.pitch + 1);
            let prev_prev_x = prev_x - png.bytes_per_pixel;
            let upper_left = if x - start > png.bytes_per_pixel - 1 && row > 0 { data[prev_prev_x] as i32 } else { 0 };
            let above = if row > 0 { data[prev_x] as i32 } else { 0 };
            let left = if x - start > png.bytes_per_pixel - 1 { data[x - png.bytes_per_pixel] as i32 } else { 0 };

            let p: i32 = left + above - upper_left;
            let pa = (p - left).abs();
            let pb = (p - above).abs();
            let pc = (p - upper_left).abs();
            if pa <= pb && pa <= pc {
                data[x] = ((data[x] as i32 + left as i32) % 256) as u8;
            } else if pb <= pc {
                data[x] = ((data[x] as i32 + above as i32) % 256) as u8;
            } else {
                data[x] = ((data[x] as i32 + upper_left as i32) % 256) as u8;
            }

            i += 1;
        }
    }
}

