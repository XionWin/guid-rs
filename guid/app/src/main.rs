use std::{vec};
mod visual;

pub use visual::*;

fn main() {
    let mut es_context = drawing::Graphic::new("/dev/dri/card1", true);
    drawing::begin_render!(render, render_frame, &mut es_context);
}

fn render(graphic: &drawing::Graphic) -> (Model, u32, std::time::SystemTime) {
    let model = Model::new();

    let mut vbos = vec![0u32; 2];

    gles::bind_vertex_array(0);

    gles::gen_buffers(vbos.len() as _, vbos.as_mut_ptr());
    gles::bind_buffer(gles::def::BufferTarget::ArrayBuffer, vbos[0]);
    gles::buffer_data(
        gles::def::BufferTarget::ArrayBuffer,
        model.get_vertex_size() as _,
        model.get_vertex_ptr(),
        gles::def::BufferUsageHint::StreamDraw
    );
    gles::bind_buffer(gles::def::BufferTarget::ElementArrayBuffer, vbos[1]);
    gles::buffer_data(
        gles::def::BufferTarget::ElementArrayBuffer,
        model.get_vertex_size() as _,
        model.get_indices_ptr(),
        gles::def::BufferUsageHint::StreamDraw
    );

    let program = gles::GfxProgram::new("shaders/simplevertshader_v3.glsl", "shaders/simplefragshader_v3.glsl");
    program.active();
    
    let pos_idx = gles::get_attrib_location(program.get_id(), "position");
    gles::vertex_attrib_pointer(
        pos_idx, 
        Model::POSTION_LENGTH as _, 
        gles::def::VertexAttribPointerType::Float, 
        false,
        model.get_stride() as _, 
        0);
    gles::enable_vertex_attrib_array(pos_idx);

    let clr_idx = gles::get_attrib_location(program.get_id(), "color");
    gles::vertex_attrib_pointer(
        clr_idx, 
        Model::COLOR_LENGTH as _, 
        gles::def::VertexAttribPointerType::Float, 
        false,
        model.get_stride() as _, 
        model.get_color_offset() as _);
    gles::enable_vertex_attrib_array(clr_idx);
    
    resize(
        graphic.get_width(), 
        graphic.get_height(), 
        gles::get_uniform_location(program.get_id(), "proj_mat")
    );

    (
        model,
        gles::get_uniform_location(program.get_id(), "model_mat"),
        std::time::SystemTime::now()
    )
}

fn render_frame(_graphic: &drawing::Graphic,  params: &mut (Model, u32, std::time::SystemTime)) {
    let (model, model_mat_location, first_tick) = params;
    let angle = (std::time::SystemTime::now().duration_since(*first_tick).unwrap().as_millis() / 20 % 360) as u32;
    let hsv = drawing::color::HSV::new(angle as f32, 1.0f32, 0.2f32);
    let rgb: drawing::color::RGB = hsv.into();
    let (r, g, b) = rgb.into();

    gles::clear_color(r as f32 / 255f32, g as f32 / 255f32, b as f32 / 255f32, 0.3f32);
    gles::clear(0x00004000);
    gles::bind_vertex_array(0);
    set_rotation_matrix(angle as f32 / 360f32 * std::f32::consts::PI * 2f32, *model_mat_location);
    // gles::draw_elements(gles::def::BeginMode::Triangles, 3, gles::def::DrawElementsType::UnsignedShort, std::ptr::null());
    gles::draw_arrays(gles::def::BeginMode::Points, 0, model.get_len() as _);
    gles::draw_arrays(gles::def::BeginMode::Polygon, 0, model.get_len() as _);
}

fn set_rotation_matrix(rad: f32, model_mat_location: u32)
{
    // rotation around z axis
    let sin_angle = rad.sin();
    let cos_angle = rad.cos();
    let mat = vec! [
        cos_angle, sin_angle, 0f32, 0f32,
        -sin_angle, cos_angle, 0f32, 0f32,
        0f32, 0f32, 1f32, 0f32,
        0f32, 0f32, 0f32, 1f32
    ];
    gles::uniform_matrix4fv(model_mat_location, 1, false, mat.as_ptr());
}

fn resize(width: i32, height: i32, proj_mat_location: u32)
{
    gles::viewport(0, 0, width, height);
    // set orthogonal view so that coordinates [-1, 1] area always visible and proportional on x and y axis
    if width > height
    {
        let f = width as f32 / height as f32;
        set_ortho_matrix(-f, f, -1f32, 1f32, -1f32, 1f32, proj_mat_location);
    }
    else
    {
        let f = height as f32 / width as f32;
        set_ortho_matrix(-1f32, 1f32, -f, f, -1f32, 1f32, proj_mat_location);
    }
}

fn set_ortho_matrix(left: f32, right: f32, bottom: f32, top: f32, n: f32, f: f32, proj_mat_location: u32) {
    // set orthogonal matrix
    let mut mat = vec![0f32; 16];
    mat[0] = 2f32 / (right - left);
    mat[1] = 0f32;
    mat[2] = 0f32;
    mat[3] = 0f32;

    mat[4] = 0f32;
    mat[5] = 2f32 / (top - bottom);
    mat[6] = 0f32;
    mat[7] = 0f32;

    mat[8] = 0f32;
    mat[9] = 0f32;
    mat[10] = -2f32 / (f - n);
    mat[11] = 0f32;

    mat[12] = -(right + left) / (right - left);
    mat[13] = -(top + bottom) / (top - bottom);
    mat[14] = -(f + n) / (f - n);
    mat[15] = 1f32;
    gles::uniform_matrix4fv(proj_mat_location, 1, false, mat.as_ptr());
}