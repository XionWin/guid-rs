use drawing::color::*;
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

    let program = gles::GfxProgram::new("shaders/simplevertshader_v3.glsl", "shaders/simplefragshader_v3.glsl");
    program.link();
    println!("program: {:#?}", program);


    loop {
        render_frame();
        context.update();
    }
}

fn render_frame() {
    gles::clear_color(1f32, 1f32, 1f32, 0.3f32);
    gles::clear(0x00004000);
}
