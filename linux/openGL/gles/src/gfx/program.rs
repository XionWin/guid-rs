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
        Self {
            id: unsafe {
                crate::ffi::glCreateProgram()
            },
            vertex_shader: super::GfxShader::new(crate::def::ShaderType::VertexShader, vertex_shader_path).load(),
            fragment_shader: super::GfxShader::new(crate::def::ShaderType::FragmentShader, fragment_shader_path).load()
        }
    }

    pub fn get_id(&self) -> c_uint {
        self.id
    }

    pub fn active(&self) {
        unsafe {
            crate::ffi::glAttachShader(self.id, self.vertex_shader.id);
            crate::ffi::glAttachShader(self.id, self.fragment_shader.id);
            crate::ffi::glLinkProgram(self.id);
            crate::ffi::glUseProgram(self.id);
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
        match get_program_linked_information(program) {
            Some(msg) => panic!("GLES program link faild error: {:?}", msg),
            None => panic!("GLES program link faild error: NONE"),
        }
    }
}

fn get_program_linked_information(program: &super::GfxProgram) -> Option<String> {
    let mut len = 0;
    unsafe {
        crate::ffi::glGetProgramiv(program.id, crate::def::ProgramParameter::InfoLogLength, &mut len);
    }
    match len {
        len if len > 0 => {
            let mut buf = vec![0u8; len as _];
            unsafe {
                crate::ffi::glGetProgramInfoLog(program.id, len, std::ptr::null_mut::<libc::c_int>(), buf.as_mut_ptr());
            }
            Some(String::from_utf8(buf).expect("GLES glGetProgramInfoLog error"))
        },
        _ => None,
    }
}