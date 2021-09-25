use libc::*;

#[derive(Debug)]
pub struct GfxShader
{
    pub(crate) id: c_uint,
    pub(crate) source: String,
    pub(crate) shader_type: crate::def::ShaderType,
}

impl GfxShader {
    pub fn new(shader_type: crate::def::ShaderType) -> Self {
        let contents = std::fs::read_to_string("shaders/simplevertshader_v3.glsl")
        .expect("Something went wrong reading the file");
        println!("{:?}", &contents);
        Self {
            id: unsafe {
                crate::ffi::glCreateShader(shader_type)
            },
            source: String::new(),
            shader_type,
        }
    }
}