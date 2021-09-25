impl crate::GfxShader {
    pub(crate) fn load(self) -> Self {
        unsafe {
            let mut source = (&self.source).bytes().collect::<Vec<libc::c_char>>();
            source.push(b'\0');
            let sources = vec![source.as_ptr()];
            crate::ffi::glShaderSource(self.id, 1, sources.as_ptr(), 0);
            crate::ffi::glCompileShader(self.id);
        }
        check(check_compile(self))
    }
}

fn check(shader: super::GfxShader) -> super::GfxShader {
    match unsafe{ crate::ffi::glGetError() } {
        code if code != 0 => panic!("GLES , error {:?}", code),
        _ => {}
    };
    shader
}

fn check_compile(shader: super::GfxShader) -> super::GfxShader {
    let mut is_compiled = 0;
    unsafe {
        crate::ffi::glGetShaderiv(shader.id, crate::def::ShaderParameter::CompileStatus, &mut is_compiled);
    }
    if is_compiled == 0 {
        panic!("GLES shader compile faild");
    }
    return shader;
}

