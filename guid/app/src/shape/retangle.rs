pub struct Retangle {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    color: (f32, f32, f32),
}

impl Retangle {
    pub fn new(x: i32, y: i32, w: i32, h: i32, color: (f32, f32, f32)) -> Self {
        Self {
            x,
            y,
            w,
            h,
            color
        }
    }
}

impl Into<super::Shape> for Retangle {
    fn into(self) -> super::Shape {
        let vertexes = vec![
            super::Vertex::new(self.x as f32 / 1080f32 * 2f32, self.y as f32 / 1920f32 * 2f32, self.color.0, self.color.1, self.color.2, 1f32),
            super::Vertex::new(self.x as f32 / 1080f32 * 2f32, (self.y + self.h) as f32 / 1920f32 * 2f32, self.color.0, self.color.1, self.color.2, 1f32),
            super::Vertex::new((self.x + self.w) as f32 / 1080f32 * 2f32, (self.y + self.h) as f32 / 1920f32 * 2f32, self.color.0, self.color.1, self.color.2, 1f32),
            super::Vertex::new((self.x + self.w) as f32 / 1080f32 * 2f32, self.y as f32 / 1920f32 * 2f32, self.color.0, self.color.1, self.color.2, 1f32),
        ];
        super::Shape::new(crate::def::VisualType::TriangleFan, vertexes)
    }
}