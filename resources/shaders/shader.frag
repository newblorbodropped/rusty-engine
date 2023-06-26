#version 140
in vec3 out_normal;
in vec3 out_position;

out vec4 color;

const vec3 light = vec3(2.0, 2.0, -1.0);

void main() {
  float brightness = dot(normalize(out_normal), normalize(light - out_position));
  color = vec4(brightness , brightness , brightness , 1.0);
}
