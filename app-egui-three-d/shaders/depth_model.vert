uniform mat4 viewProjection;
in vec3 position;
out vec3 pos;

void main()
{
    pos = position;
    gl_Position = viewProjection * vec4(position, 1.0);
    // gl_Position = vec4(position, 1.0);
}
