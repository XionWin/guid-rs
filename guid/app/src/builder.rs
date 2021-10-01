use gles::GfxProgram;

use crate::{Shape, Visual};

pub struct Builder {
    program: GfxProgram,
}

impl Builder {
    pub fn new() -> Self {
        let program = gles::GfxProgram::new("shaders/simplevertshader_v3.glsl", "shaders/simplefragshader_v3.glsl");
        program.active();
        Self {
            program
        }
    }

    pub fn build_shape(&self, shape: Shape) -> Visual {
        build_shape(&self.program, shape)
    }
}

fn build_shape(program: &GfxProgram, shape: Shape) -> Visual {
    
    let mut vbos = vec![0u32; 2];

    let mut vao = 0u32;
    gles::gen_vertex_arrays(1, &mut vao);
    println!("vao: {:?}", vao);

    gles::bind_vertex_array(vao);

    gles::gen_buffers(vbos.len() as _, vbos.as_mut_ptr());
    gles::bind_buffer(gles::def::BufferTarget::ArrayBuffer, vbos[0]);
    gles::buffer_data(
        gles::def::BufferTarget::ArrayBuffer,
        shape.get_vertex_size() as _,
        shape.get_vertex_ptr(),
        gles::def::BufferUsageHint::StreamDraw
    );
    gles::bind_buffer(gles::def::BufferTarget::ElementArrayBuffer, vbos[1]);
    gles::buffer_data(
        gles::def::BufferTarget::ElementArrayBuffer,
        shape.get_vertex_size() as _,
        shape.get_indices_ptr(),
        gles::def::BufferUsageHint::StreamDraw
    );

    
    let pos_idx = gles::get_attrib_location(program.get_id(), "position");
    gles::vertex_attrib_pointer(
        pos_idx, 
        Shape::POSTION_LENGTH as _, 
        gles::def::VertexAttribPointerType::Float, 
        false,
        shape.get_stride() as _, 
        0);
    gles::enable_vertex_attrib_array(pos_idx);

    let clr_idx = gles::get_attrib_location(program.get_id(), "color");
    gles::vertex_attrib_pointer(
        clr_idx, 
        Shape::COLOR_LENGTH as _, 
        gles::def::VertexAttribPointerType::Float, 
        false,
        shape.get_stride() as _, 
        shape.get_color_offset() as _);
    gles::enable_vertex_attrib_array(clr_idx);

    let proj_mat = gles::get_uniform_location(program.get_id(), "proj_mat");
    let model_mat = gles::get_uniform_location(program.get_id(), "model_mat");
    
    Visual::new(vao, shape.get_shape_type(), shape.get_visual_type(), shape.get_len(), pos_idx, clr_idx, proj_mat, model_mat)
}