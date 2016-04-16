use std::fmt;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            r: r,
            g: g,
            b: b,
            a: a
        }
    }
}

impl Clone for Color {
    fn clone(&self) -> Self {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.r = source.r;
        self.g = source.g;
        self.b = source.b;
        self.a = source.a;
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "R: {}, G: {}, B: {}, A: {}", self.r, self.g, self.b, self.a);
        Ok(())
    }
}
