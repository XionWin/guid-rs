#[repr(i32)]
pub enum SurfaceType
{
    OpenGL = crate::def::Definition::OPENGL_BIT,
    // OpenGLES = Definition.OPENGL_ES_BIT,
    // OpenOpenGLES = Definition.OPENGL_ES2_BIT,
    // OpenGLESV3 = Definition.OPENGL_ES3_BIT,

    OpenGLES = crate::def::Definition::OPENGL_ES2_BIT,
    // Window = crate::def::Definition::WINDOW_BIT,
}