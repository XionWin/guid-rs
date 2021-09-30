#[repr(u32)]
pub enum GLFeature {
    Blend = 0x0BE2,
    CullFace = 0x0B45,
    //GL_DEBUG_OUTPUT = 0x88E2,
    //GL_DEBUG_OUTPUT_SYNCHRONOUS = 0x88E4,
    DepthTest = 0x0B71,
    Dither = 0x0BD0,
    PolygonOffsetFill = 0x8037,
    //GL_PRIMITIVE_RESTART_FIXED_INDEX = 0x88E9,
    //GL_RASTERIZER_DISCARD = 0x88EA,
    SampleAlphaToCoverage = 0x809E,
    SampleCoverage = 0x80A0,
    //GL_SAMPLE_MASK = 0x88EA,
    ScissorTest = 0x0C11,
    StencilTest = 0x0B90,
}