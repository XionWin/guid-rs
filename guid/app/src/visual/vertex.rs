
type VertexDataType = libc::c_float;

#[repr(C)]
#[derive(Debug)]
pub struct Vertex
{
    pub x: VertexDataType,
    pub y: VertexDataType,
    pub r: VertexDataType,
    pub g: VertexDataType,
    pub b: VertexDataType,
    pub a: VertexDataType,
}

impl Vertex {
    pub fn new(x: VertexDataType, y: VertexDataType, r: VertexDataType, g: VertexDataType, b: VertexDataType, a: VertexDataType) -> Self {
        Self {x, y, r, g, b, a}
    }
}