use super::RGBA;

pub struct HSVA {
    pub(crate) h: f32,
    pub(crate) s: f32,
    pub(crate) v: f32,
    pub(crate) a: f32,
}

impl HSVA {
    pub fn new(h: f32, s: f32, v: f32, a: f32) -> Self {
        Self {
            h,
            s,
            v,
            a,
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

    pub fn get_a(&self) -> f32 {
        self.a
    }
}


impl Into<RGBA> for HSVA {
    fn into(self) -> RGBA {
        todo!()
    }
}