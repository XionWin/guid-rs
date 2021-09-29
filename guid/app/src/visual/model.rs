pub struct Model {
    vertexes: Vec<super::Vertex>,
    indices: Vec<u16>,
    stride: usize,
    color_offset: usize,
}

impl Model {
    const TRIANGLE_SIZE: f32 = 0.85f32;
    pub const POSTION_LENGTH: usize = 2;
    pub const COLOR_LENGTH: usize = 4;

    pub fn new() -> Self {
        let mut vertexes = vec![
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
            super::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
            super::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
            super::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
        ];
    
        for i in 0..vertexes.len() {
            vertexes[i].x = (-(i as f32) * (2f32 * std::f32::consts::PI / vertexes.len() as f32)).cos() * Model::TRIANGLE_SIZE;
            vertexes[i].y = (-(i as f32) * (2f32 * std::f32::consts::PI / vertexes.len() as f32)).sin() * Model::TRIANGLE_SIZE;
            
            let angle = ((i as f32) * (360f32 / vertexes.len() as f32)) as u32;
            let hsv = drawing::color::HSV::new(angle as f32, 1.0f32, 0.2f32);
            let rgb: drawing::color::RGB = hsv.into();
            let (r, g, b) = rgb.into();
            vertexes[i].r = r as f32 /255f32;
            vertexes[i].g = g as f32 /255f32;
            vertexes[i].b = b as f32 /255f32;
        }
    
        let indices = vec![0u16, 1u16, 2u16];

        Self {
            vertexes,
            indices,
            stride: std::mem::size_of::<super::Vertex>(),
            color_offset: std::mem::size_of::<f32>() * 2,
        }
    }
    
    pub fn get_len(&self) -> usize {
        self.vertexes.len()
    }

    pub fn get_vertex_size(&self) -> usize {
        std::mem::size_of::<super::Vertex>() * self.vertexes.len()
    }

    pub fn get_vertex_ptr(&self) -> *const libc::c_void {
        self.vertexes.as_ptr() as _
    }

    pub fn get_indices_size(&self) -> usize {
        std::mem::size_of::<u16>() * self.indices.len()
    }

    pub fn get_indices_ptr(&self) -> *const libc::c_void {
        self.indices.as_ptr() as _
    }

    pub fn get_stride(&self) -> usize {
        self.stride
    }

    pub fn get_color_offset(&self) -> usize {
        self.color_offset
    }
}