#version 300 es
layout(location = 0) in vec2 position;
layout(location = 2) in vec4 color;
out vec4 vColor;
uniform mat4 proj_mat;
uniform mat4 model_mat;
void main()
{
        gl_Position = proj_mat * model_mat * vec4(position, 0, 1.0);
        gl_PointSize = 10.0;
        vColor = color;
}