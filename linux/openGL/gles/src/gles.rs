use std::{ffi::CStr};

use libc::*;

pub fn check_error() -> c_uint {
    unsafe{
        crate::ffi::glGetError()
    }
}

pub fn get_string(name: crate::def::StringName) -> String {
    String::from(unsafe {
        CStr::from_ptr(crate::ffi::glGetString(name))
    }.to_str().unwrap())
}

pub fn clear_color(red: c_float, green: c_float, blue: c_float, alpha: c_float) {
    unsafe {
        crate::ffi::glClearColor(red, green, blue, alpha);
    }
}
pub fn clear(mask: c_uint) {
    unsafe {
        crate::ffi::glClear(mask);
    }
}

pub fn viewport(x: c_int, y: c_int, width: c_int, height: c_int) {
    unsafe {
        crate::ffi::glViewport(x, y, width, height);
    }
}

pub fn gen_vertex_arrays(n: c_int, array: *mut c_uint) {
    unsafe {
        crate::ffi::glGenVertexArrays(n, array);
    }
}

pub fn gen_buffers(n: c_uint, buffer: *mut c_uint) {
    unsafe {
        crate::ffi::glGenBuffers(n, buffer);
    }
}

pub fn bind_vertex_array(array_id: c_uint) {
    unsafe {
        crate::ffi::glBindVertexArray(array_id);
    }
}

pub fn bind_buffer(target: crate::def::BufferTarget, buffer_id: c_uint) {
    unsafe {
        crate::ffi::glBindBuffer(target, buffer_id);
    }
}

pub fn buffer_data(target: crate::def::BufferTarget, size: c_int, data: *const c_void, hint: crate::def::BufferUsageHint) {
    unsafe {
        crate::ffi::glBufferData(target, size, data, hint);
    }
}

pub fn uniform_matrix4fv(location: c_uint, count: c_uint, transpose: bool, value: *const c_float) {
    unsafe {
        crate::ffi::glUniformMatrix4fv(location, count, transpose, value);
    }
}


pub fn use_program(program_id: c_uint) {
    unsafe {
        crate::ffi::glUseProgram(program_id);
    }
}

pub fn get_attrib_location(program_id: c_uint, name: &str) -> c_uint {
    let mut buffer = name.bytes().collect::<Vec<u8>>();
    buffer.push(b'\0');
    match unsafe { crate::ffi::glGetAttribLocation(program_id, buffer.as_ptr()) } {
        value if value >= 0 => value as c_uint,
        _ => panic!("GLES get_attrib_location error")
    }
}

pub fn get_uniform_location(program_id: c_uint, name: &str) -> c_uint {
    let mut buffer = name.bytes().collect::<Vec<u8>>();
    buffer.push(b'\0');
    match unsafe { crate::ffi::glGetUniformLocation(program_id, buffer.as_ptr()) } {
        value if value >= 0 => value as c_uint,
        _ => panic!("GLES get_uniform_location error")
    }
}

pub fn enable_vertex_attrib_array(index: c_uint) {
    unsafe {
        crate::ffi::glEnableVertexAttribArray(index)
    }
}

pub fn vertex_attrib_pointer(
    index: c_uint, 
    size: c_int, 
    attrib_pointer_type: crate::def::VertexAttribPointerType, 
    is_normalized: bool, 
    stride: c_uint, 
    offset: c_int) {
    unsafe {
        crate::ffi::glVertexAttribPointer(index, size, attrib_pointer_type, is_normalized, stride, offset as _);
    }
}

pub fn draw_elements(
    begin_mode: crate::def::BeginMode,
    count: c_uint,
    draw_type: crate::def::DrawElementsType,
    indices: *const c_void
) {
    unsafe {
        crate::ffi::glDrawElements(begin_mode, count, draw_type, indices);
    }
}
    
