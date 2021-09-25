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
    let mut counter = 0u64;
    let mut value = 0f32;
    let mut direction = true;

    let shader = gles::GfxShader::new(gles::def::ShaderType::VertexShader);
    println!("shader: {:#?}", shader);

    let mut last_tick = std::time::SystemTime::now();
    loop {
        render_frame(&mut value);
        context.update();

        value += match direction {
            true => 1f32,
            false => -1f32,
        };

        match value {
            v if v >= 360f32 => direction = false,
            v if v <= 0f32 => direction = true,
            _ => {},
        }

        counter += 1;

        match last_tick.elapsed() {
            Ok(elapsed) if elapsed.as_secs() > 1 => {
                let fps = counter as f64 / elapsed.as_millis() as f64 * 1000f64;
                println!("{:?} frames rendered in {:?} millis -> FPS= {:.2?}", counter, elapsed.as_millis(), fps);
                counter = 0;
                last_tick = std::time::SystemTime::now();
            }
            _ => {}
        }
    }
}

fn render_frame(angle: &mut f32) {
    let hsv = HSV::new(*angle, 1.0f32, 0.5f32);
    let rgb: RGB = hsv.into();
    let (r, g, b) = rgb.into();
    gles::clear_color(r as f32 / 255f32, g as f32 / 255f32, b as f32 / 255f32, 0.3f32);
    gles::clear(0x00004000);
}
