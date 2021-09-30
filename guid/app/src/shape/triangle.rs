pub struct Triangle {
    p1: (i32, i32),
    p2: (i32, i32),
    p3: (i32, i32),
    color: (f32, f32, f32),
}

impl Triangle {
    pub fn new(p1: (i32, i32), p2: (i32, i32), p3: (i32, i32), color: (f32, f32, f32)) -> Self {
        Self {
            p1,
            p2,
            p3,
            color
        }
    }
}

impl Into<super::Shape> for Triangle {
    fn into(self) -> super::Shape {
        let vertexes = vec![
            super::Vertex::new(self.p1.0 as f32 / 1080f32 * 2f32 -1f32, self.p1.1 as f32 / 1920f32 * 2f32 - 1f32, self.color.0, self.color.1, self.color.2, 1f32),
            super::Vertex::new(self.p2.0 as f32 / 1080f32 * 2f32 -1f32, self.p2.1 as f32 / 1920f32 * 2f32 - 1f32, self.color.0, self.color.1, self.color.2, 1f32),
            super::Vertex::new(self.p3.0 as f32 / 1080f32 * 2f32 -1f32, self.p3.1 as f32 / 1920f32 * 2f32 - 1f32, self.color.0, self.color.1, self.color.2, 1f32),
        ];
        super::Shape::new(crate::def::VisualType::Triangles, vertexes)
    }
}