use std::{ffi::CStr};

use libc::*;

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
pub fn clear(mask: c_int) {
    unsafe {
        crate::ffi::glClear(mask);
    }
}

pub fn viewport(x: c_int, y: c_int, width: c_int, height: c_int) {
    unsafe {
        crate::ffi::glViewport(x, y, width, height);
    }
}

pub fn gen_vertex_arrays(n: c_uint, array: *mut c_uint) {
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