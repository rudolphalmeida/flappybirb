#version 330 core

in vec2 sprite_uv;

uniform sampler2D sprite;
uniform vec2 pan;

out vec4 FragColor;

void main() {
    FragColor = texture(sprite, sprite_uv + pan);
    if (FragColor.a <= 0.0) {
        discard;
    }
}
