#version 140
in vec2 out_position;

uniform sampler2D color_sampler;
uniform vec2 resolution;

out vec4 color;

vec2 tex_coords() {
  return (0.5 * out_position) + vec2(0.5, 0.5);
}

void main() {
  color = texture(color_sampler, tex_coords());
}
