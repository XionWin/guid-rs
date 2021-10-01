use std::vec;

pub struct Shape {
    shape_type: crate::def::ShapeType,
    visual_type: crate::def::VisualType,
    vertexes: Vec<super::Vertex>,
    indices: Vec<u16>,
    stride: usize,
    color_offset: usize,
}

impl Shape {
    pub const POSTION_LENGTH: usize = 2;
    pub const COLOR_LENGTH: usize = 4;

    pub fn new(shape_type: crate::def::ShapeType, visual_type: crate::def::VisualType, vertexes: Vec<super::Vertex>) -> Self {
        Self {
            shape_type,
            visual_type,
            vertexes,
            indices: vec![0u16, 1u16, 2u16],
            stride: std::mem::size_of::<super::Vertex>(),
            color_offset: std::mem::size_of::<f32>() * 2,
        }
    }

    pub fn get_shape_type(&self) -> crate::def::ShapeType {
        self.shape_type
    }
    
    pub fn get_visual_type(&self) -> crate::def::VisualType {
        self.visual_type
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