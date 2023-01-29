in vec3 pos;

layout (location = 0) out vec4 outColor;

void main()
{
    // outColor = vec4(pos * 2.0, 1.0);
    // outColor = vec4(1.0, 1.0, 1.0, 1.0);
    outColor = vec4(1.0, 0.0, 0.0, 1.0);
}
