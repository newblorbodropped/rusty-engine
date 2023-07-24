#version 140
in vec2 out_position;

uniform sampler2D tex;

out vec4 color;

vec2 tex_coords() {
  return (0.5 * out_position) + vec2(0.5, 0.5);
}

void main() {
  color = texture(tex, tex_coords());
}
