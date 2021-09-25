#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    FragmentShader = 0x8B30,
    VertexShader = 0x8B31,
    GeometryShader = 0x8DD9,
    // GeometryShaderExt = 0x8DD9,
    TessEvaluationShader = 0x8E87,
    TessControlShader = 0x8E88,
    // Manual added
    ComputeShader = 0x91B9,
}