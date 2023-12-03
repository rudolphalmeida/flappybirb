#version 330 core

in vec2 sprite_uv;

uniform sampler2D sprite;

out vec4 FragColor;

void main() {
    FragColor = texture(sprite, sprite_uv);
}
