use std::io::SeekFrom;

use libc::*;

#[derive(Debug)]
pub struct GfxProgram
{
    pub(crate) id: c_uint,
    pub(crate) vertex_shader: super::GfxShader,
    pub(crate) fragment_shader: super::GfxShader,
}

impl GfxProgram {
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> Self {
        let vertex_shader = super::GfxShader::new(crate::def::ShaderType::VertexShader, vertex_shader_path).load();
        let fragment_shader = super::GfxShader::new(crate::def::ShaderType::FragmentShader, fragment_shader_path).load();
        Self {
            id: unsafe {
                crate::ffi::glCreateProgram()
            },
            vertex_shader,
            fragment_shader,
        }
    }

    pub fn link(&self) {
        unsafe {
            crate::ffi::glAttachShader(self.id, self.vertex_shader.id);
            crate::ffi::glAttachShader(self.id, self.fragment_shader.id);
            crate::ffi::glLinkProgram(self.id);
            check_link(self);
        }
    }
}

impl Drop for GfxProgram {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::glDeleteShader(self.id);
        }
    }
}

fn check_link(program: &super::GfxProgram) {
    let mut is_linked = 0;
    unsafe {
        crate::ffi::glGetProgramiv(program.id, crate::def::ProgramParameter::LinkStatus, &mut is_linked);
    }
    if is_linked == 0 {
        panic!("GLES program link faild");
    }
}