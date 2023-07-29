#version 140
in vec3 out_normal;
in vec3 out_position;
in vec2 out_tex_coords;
in vec3 light;

uniform sampler2D tex;

out vec4 color;

void main() {
  float diffuse = max(0.05, dot(normalize(out_normal), normalize(light - out_position)));

  vec3 light_dir = normalize(light - out_position);
  vec3 view_dir = normalize(-out_position);
  vec3 half_dir = normalize(light_dir + view_dir);
  
  float specular = pow(max(dot(normalize(out_normal), half_dir), 0.0), 16.0);
  
  color = (specular + diffuse) * texture(tex, out_tex_coords);
}
