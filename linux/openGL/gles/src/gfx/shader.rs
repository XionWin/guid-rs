use libc::*;

#[derive(Debug)]
pub struct GfxShader
{
    pub(crate) id: c_uint,
    pub(crate) source: String,
    pub(crate) shader_type: crate::def::ShaderType,
}

impl GfxShader {
    pub fn new(shader_type: crate::def::ShaderType, path: &str) -> Self {
        Self {
            id: unsafe {
                crate::ffi::glCreateShader(shader_type)
            },
            source: std::fs::read_to_string(path)
            .expect("Something went wrong reading the file"),
            shader_type,
        }
    }
}

impl Drop for GfxShader {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::glDeleteShader(self.id);
        }
    }
}