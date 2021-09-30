use std::{vec};
mod shape;
mod visual;
mod builder;
pub mod def;

pub use shape::*;
pub use visual::*;
pub use builder::*;

fn main() {
    let mut es_context = drawing::Graphic::new("/dev/dri/card1", true);
    drawing::begin_render!(render, render_frame, &mut es_context);
}

fn render(graphic: &drawing::Graphic) -> (Vec<Visual>, std::time::SystemTime) {
    let builder = Builder::new();
    let triangle_1 = builder.build_shape(
        crate::Triangle::new(
            (540, 0),
            (0, 1920),
            (1080, 1920),
            (1f32, 0f32, 0f32)
        ).into()
    );

    let triangle_2 = builder.build_shape(
        crate::Triangle::new(
            (540, 1920),
            (0, 0),
            (1080, 0),
            (0f32, 0f32, 1f32)
        ).into()
    );

    resize(
        graphic.get_width(), 
        graphic.get_height(), 
        &triangle_1
    );
    resize(
        graphic.get_width(), 
        graphic.get_height(), 
        &triangle_2
    );

    gles::set_line_width(10f32);
    
    (
        vec![triangle_1, triangle_2],
        std::time::SystemTime::now()
    )
}

fn render_frame(_graphic: &drawing::Graphic,  params: &mut (Vec<Visual>, std::time::SystemTime)) {
    let (visuals, _first_tick) = params;
    let angle = 0; //(std::time::SystemTime::now().duration_since(*first_tick).unwrap().as_millis() / 20 % 360) as u32;
    // let hsv = drawing::color::HSV::new(angle as f32, 1.0f32, 0.2f32);
    // let rgb: drawing::color::RGB = hsv.into();
    // let (r, g, b) = rgb.into();
    
    // gles::clear_color(r as f32 / 255f32, g as f32 / 255f32, b as f32 / 255f32, 0.3f32);
    gles::clear_color(0f32, 0f32, 0f32, 1f32);
    gles::clear(0x00004000);
    
    for visual in visuals {


        gles::bind_vertex_array(visual.get_id());
  

        set_rotation_matrix(angle as f32 / 360f32 * std::f32::consts::PI * 2f32, visual.get_model_mat_id());
        // gles::draw_elements(gles::def::BeginMode::Triangles, 3, gles::def::DrawElementsType::UnsignedShort, std::ptr::null());
        match visual.get_visual_type() {
            crate::def::VisualType::Triangles => gles::draw_arrays(gles::def::BeginMode::Triangles, 0, visual.get_len() as _),
            crate::def::VisualType::Lines => gles::draw_arrays(gles::def::BeginMode::LineStrip, 0, visual.get_len() as _),
        }
        gles::draw_arrays(gles::def::BeginMode::Points, 0, visual.get_len() as _);
    }
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

fn resize(width: i32, height: i32, visual: &Visual)
{
    gles::viewport(0, 0, width, height);
    // set orthogonal view so that coordinates [-1, 1] area always visible and proportional on x and y axis
    if width > height
    {
        let f = width as f32 / height as f32;
        set_ortho_matrix(-f, f, -1f32, 1f32, -1f32, 1f32, visual.get_proj_mat_id());
    }
    else
    {
        let f = height as f32 / width as f32;
        set_ortho_matrix(-1f32, 1f32, -f, f, -1f32, 1f32, visual.get_proj_mat_id());
    }
}

fn set_ortho_matrix(_left: f32, _right: f32, _bottom: f32, _top: f32, _n: f32, _f: f32, proj_mat_location: u32) {
    // set orthogonal matrix
    // let mut mat = vec![0f32; 16];
    // mat[0] = 2f32 / (right - left);
    // mat[1] = 0f32;
    // mat[2] = 0f32;
    // mat[3] = 0f32;

    // mat[4] = 0f32;
    // mat[5] = 2f32 / (top - bottom);
    // mat[6] = 0f32;
    // mat[7] = 0f32;

    // mat[8] = 0f32;
    // mat[9] = 0f32;
    // mat[10] = -2f32 / (f - n);
    // mat[11] = 0f32;

    // mat[12] = -(right + left) / (right - left);
    // mat[13] = -(top + bottom) / (top - bottom);
    // mat[14] = -(f + n) / (f - n);
    // mat[15] = 1f32;
    
    let mat = vec! [
        1f32, 0f32, 0f32, 0f32,
        0f32, 1f32, 0f32, 0f32,
        0f32, 0f32, 1f32, 0f32,
        0f32, 0f32, 0f32, 1f32
    ];
    gles::uniform_matrix4fv(proj_mat_location, 1, false, mat.as_ptr());
}