#[derive(Debug)]
pub struct HSV {
    pub(crate) h: f32,
    pub(crate) s: f32,
    pub(crate) v: f32,
}

impl HSV {
    pub fn new(h: f32, s: f32, v: f32) -> Self {
        Self {
            h,
            s,
            v,
        }
    }

    pub fn get_h(&self) -> f32 {
        self.h
    }

    pub fn get_s(&self) -> f32 {
        self.s
    }

    pub fn get_v(&self) -> f32 {
        self.v
    }
}

impl Into<super::RGB> for HSV {
    fn into(self) -> super::RGB {
        super::extension::hsv2rgb(&self)
    }
}