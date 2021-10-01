pub struct Visual {
    id: u32,
    shape_type: crate::def::ShapeType,
    visual_type: crate::def::VisualType,
    len: usize,
    vertex_id: u32,
    color_id: u32,
    proj_mat_id: u32,
    model_mat_id: u32,
}

impl Visual {
    pub fn new(id: u32, shape_type: crate::def::ShapeType, visual_type: crate::def::VisualType, len: usize, vertex_id: u32, color_id: u32, proj_mat_id: u32, model_mat_id: u32) -> Self {
        Self {
            id, 
            shape_type,
            visual_type,
            len,
            vertex_id,
            color_id,
            proj_mat_id,
            model_mat_id,
        }
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_shape_type(&self) -> crate::def::ShapeType {
        self.shape_type
    }

    pub fn get_visual_type(&self) -> crate::def::VisualType {
        self.visual_type
    }

    pub fn get_len(&self) -> usize {
        self.len
    }

    pub fn get_vertex_id(&self) -> u32 {
        self.vertex_id
    }

    pub fn get_color_id(&self) -> u32 {
        self.color_id
    }

    pub fn get_proj_mat_id(&self) -> u32 {
        self.proj_mat_id
    }

    pub fn get_model_mat_id(&self) -> u32 {
        self.model_mat_id
    }
}
