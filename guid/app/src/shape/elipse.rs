pub struct Elipse {
    cx: i32,
    cy: i32,
    r1: i32,
    r2: i32,
    color: (f32, f32, f32),
}

impl Elipse {
    pub fn new( cx: i32, cy: i32, r1: i32, r2: i32, color: (f32, f32, f32)) -> Self {
        Self {
            cx,
            cy,
            r1,
            r2,
            color
        }
    }
}

impl Into<super::Shape> for Elipse {
    fn into(self) -> super::Shape {
        let mut vertexes = Vec::<super::Vertex>::new();
        for i in 0..360 {
            let x = self.r1 as f32 * (i as f32).cos() + self.cx as f32;
            let y = self.r2 as f32 * (i as f32).sin() + self.cy as f32;
            vertexes.push(super::Vertex::new(x / 1080f32 * 2f32, y / 1920f32 * 2f32, self.color.0, self.color.1, self.color.2, 1f32));
        }
        super::Shape::new(crate::def::ShapeType::Elipse, crate::def::VisualType::TriangleFan, vertexes)
    }
}