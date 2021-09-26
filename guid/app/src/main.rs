use std::os::raw::c_float;

use egl::Context;

fn main() -> ! {
    let fd = libc::File::new("/dev/dri/card1").get_fd();
    let r = drm::Resources::new(fd);
    println!("{:#?}", r);
    
    let drm = drm::Drm::new(
        r,
        |conn| conn.get_connection_status() == drm::def::ConnectionStatus::Connected
    );

    let mode = drm.get_mode();
    println!("{:#?}", drm);
    println!("{:#?}", mode);

    let gbm = gbm::Gbm::new(drm, gbm::def::SurfaceFormat::ARGB8888, vec![gbm::def::FormatModifier::DRM_FORMAT_MOD_LINEAR]);
    println!("{:#?}", gbm);

    for surface_format in gbm::def::SurfaceFormat::iter() {
        if gbm.get_surface().get_device().is_format_supported(surface_format, gbm::def::SurfaceFlags::Linear) {
            println!("{:?}", surface_format);
        } 
    }

    let mut context = egl::Context::new(gbm, true);
    println!("{:#?}", context);

    println!("GL Extensions: {:?}", gles::get_string(gles::def::StringName::Extensions));
    println!("GL Version: {:?}", gles::get_string(gles::def::StringName::Version));
    println!("GL Sharding Language Version: {:?}", gles::get_string(gles::def::StringName::ShadingLanguageVersion));
    println!("GL Vendor: {:?}", gles::get_string(gles::def::StringName::Vendor));
    println!("GL Renderer: {:?}", gles::get_string(gles::def::StringName::Renderer));

    context.initialize();
    render(context);
}

fn render(mut context: Context) -> ! {

    gles::viewport(0, 0, context.get_width(), context.get_height());
    
    const TRIANGLE_SIZE: f32 = 0.8f32;
    let mut vertexes = vec![
        gles::Vertex::new(-0.5f32, 0.5f32, 1f32, 0f32, 0f32, 1f32),
        gles::Vertex::new(-0.5f32, -0.5f32, 0f32, 1f32, 0f32, 1f32),
        gles::Vertex::new(0.5f32, -0.5f32, 0f32, 0f32, 1f32, 1f32),
    ];

    for i in 0..vertexes.len() {
        vertexes[i].x = (-(i as f32) * (2f32 * std::f32::consts::PI / vertexes.len() as f32)).cos() * TRIANGLE_SIZE;
        vertexes[i].y = (-(i as f32) * (2f32 * std::f32::consts::PI / vertexes.len() as f32)).sin() * TRIANGLE_SIZE;
    }

    let indices = vec![0u16, 1u16, 2u16];

    let mut vbos = vec![0u32; 2];

    gles::bind_vertex_array(0);

    gles::gen_buffers(vbos.len() as _, vbos.as_mut_ptr());
    gles::bind_buffer(gles::def::BufferTarget::ArrayBuffer, vbos[0]);
    gles::buffer_data(
        gles::def::BufferTarget::ArrayBuffer,
        (std::mem::size_of::<gles::Vertex>() * vertexes.len()) as _,
        vertexes.as_ptr() as _,
        gles::def::BufferUsageHint::StreamDraw
    );
    gles::bind_buffer(gles::def::BufferTarget::ElementArrayBuffer, vbos[1]);
    gles::buffer_data(
        gles::def::BufferTarget::ElementArrayBuffer,
        (std::mem::size_of::<u16>() * indices.len()) as _,
        indices.as_ptr() as _,
        gles::def::BufferUsageHint::StreamDraw
    );

    let program = gles::GfxProgram::new("shaders/simplevertshader_v3.glsl", "shaders/simplefragshader_v3.glsl");
    program.active();
    
    let po_attrib_index = gles::get_attrib_location(program.get_id(), "position");
    gles::vertex_attrib_pointer(
        po_attrib_index, 
        2, 
        gles::def::VertexAttribPointerType::Float, 
        false,
        (std::mem::size_of::<gles::Vertex>()) as _, 
        0);
    gles::enable_vertex_attrib_array(po_attrib_index);

    let co_attrib_index = gles::get_attrib_location(program.get_id(), "color");
    gles::vertex_attrib_pointer(
        co_attrib_index, 
        4, 
        gles::def::VertexAttribPointerType::Float, 
        false,
        (std::mem::size_of::<gles::Vertex>()) as _, 
        (std::mem::size_of::<f32>() * 2) as _);
    gles::enable_vertex_attrib_array(co_attrib_index);
    

    gles::viewport(0, 0, context.get_width(), context.get_height());

    loop {
        
        gles::viewport(0, 0, context.get_width(), context.get_height());
        gles::clear_color(1f32, 1f32, 1f32, 0.3f32);
        gles::clear(0x00004000);
        gles::bind_vertex_array(0);
        gles::draw_elements(gles::def::BeginMode::Triangles, 6, gles::def::DrawElementsType::UnsignedShort, std::ptr::null());

        context.update();
    }
}
