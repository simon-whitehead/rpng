
use png::PngFile;

pub trait Filter {
    fn apply(&self, data: &mut [u8], start: usize, png: &PngFile);
}

pub struct None;
impl Filter for None {
    fn apply(&self, _data: &mut [u8], _start: usize, _png: &PngFile) {
        // No op
    }
}

pub struct Sub;
impl Filter for Sub {
    fn apply(&self, data: &mut [u8], start: usize, png: &PngFile) {
        let mut i = 0;
        while i < png.pitch {
            let x = start + i;
            if x - start > png.bytes_per_pixel {
                let result = data[x] as u32 + data[x - png.bytes_per_pixel] as u32;
                data[x] = result as u8;
            }

            i += 1;
        }
    }
}

// pub struct Up;
// impl Filter for Up {
//     fn filter(&self, data: &mut [u8], row_size: u32) {
//     }
// }
