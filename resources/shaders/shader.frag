#version 140
in vec3 out_normal;
in vec3 out_position;
in vec2 out_tex_coords;

uniform sampler2D tex;

out vec4 color;

const vec3 light = vec3(7.0, 15.0, -1.0);

void main() {
  float brightness = max(0.05, dot(normalize(out_normal), normalize(light - out_position)));
  color = brightness * texture(tex, out_tex_coords);
}
