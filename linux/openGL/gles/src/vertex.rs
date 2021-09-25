use libc::*;

#[repr(C)]
#[derive(Debug)]
pub struct Vertex
{
    pub x: c_float,
    pub y: c_float,
    pub r: c_float,
    pub g: c_float,
    pub b: c_float,
    pub a: c_float,
}