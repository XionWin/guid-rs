#[derive(Debug)]
pub struct RGB {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
        }
    }

    pub fn get_r(&self) -> u8 {
        self.r
    }

    pub fn get_g(&self) -> u8 {
        self.g
    }

    pub fn get_b(&self) -> u8 {
        self.b
    }
}

impl Into<(u8, u8, u8)> for RGB {
    fn into(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}