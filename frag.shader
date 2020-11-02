#version 330 core

out vec4 Color;

in vec3 ourColor;
in vec2 TexCoord;

uniform sampler2D ourTexture;

void main()
{
    Color = texture(ourTexture, TexCoord * vec2(1.0, -1.0)) * vec4(ourColor, 1.0);
}
