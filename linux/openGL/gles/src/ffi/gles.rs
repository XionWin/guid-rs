use libc::*;

#[link(name = "GLESv2")]
#[allow(improper_ctypes)]
extern "C" {

    pub fn glGetError() -> c_uint;
    pub fn glGetString(name: crate::def::StringName) -> *const c_char;
    pub fn glClearColor(red: c_float, green: c_float, blue: c_float, alpha: c_float);
    pub fn glClear(mask: c_int);
    pub fn glViewport(x: c_int, y: c_int, width: c_int, height: c_int);
    
    pub fn glGenVertexArrays(n: c_uint, array: *mut c_uint);
    pub fn glGenBuffers(n: c_uint, buffer: *mut c_uint);
    pub fn glBindVertexArray(array_id: c_uint);
    pub fn glBindBuffer(target: crate::def::BufferTarget, buffer_id: c_uint);
    pub fn glBufferData(target: crate::def::BufferTarget, size: c_int, data: *const c_void, hint: crate::def::BufferUsageHint);
    pub fn glUniformMatrix4fv(location: c_uint, count: c_uint, transpose: bool, value: *const c_float);

    pub fn glCreateProgram() -> c_uint;
    pub fn glGetProgramiv(program_id: c_uint, program_parameter: crate::def::ProgramParameter, value: *mut c_int);
    pub fn glCreateShader(shader_type: crate::def::ShaderType) -> c_uint;
    pub fn glDeleteShader(shader_id: c_uint);
    pub fn glShaderSource(shader_id: c_uint, count: c_int, source: *const *const c_char, len: c_int);
    pub fn glCompileShader(shader_id: c_uint);
    pub fn glGetShaderiv(shader_id: c_uint, shader_parameter: crate::def::ShaderParameter, value: *mut c_int);

    
    pub fn glAttachShader(program_id: c_uint, shader_id: c_uint);
    pub fn glLinkProgram(program_id: c_uint);
    pub fn glGetProgramInfoLog(program_id: c_uint, buf_size: c_int, length: *mut c_int, info: *mut c_uchar);
}
