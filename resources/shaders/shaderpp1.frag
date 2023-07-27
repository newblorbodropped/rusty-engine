#version 140

in vec2 out_position;

uniform sampler2D color_sampler;
uniform float time;
uniform vec2 resolution;

out vec4 color;

float white_noise(vec2 xy, float seed) {
  float PHI = 100.61803398874989484820459;
  return fract(tan(distance(xy * PHI, xy) * seed) * xy.x);
}

vec2 crt_pos(vec2 pos) {
  float offset_x = 0.1 * pos.x * pos.x * pos.x * pos.y * pos.y;
  float offset_y = 0.1 * pos.y * pos.y * pos.y * pos.x * pos.x ;
  float crtx = pos.x + offset_x;
  float crty = pos.y + offset_y;
  return vec2(crtx, crty);
}

float spherical_decay(float x) {
  return pow(1 - pow(x, 6.0), 1.0 / 6.0);
}

void main() {
  vec2 image_pos = crt_pos(out_position) * 0.5 + 0.5 ;
  vec2 image_step_x = 2.0 * vec2(1.0 / resolution.x, 0.0);
  vec2 image_step_y = 2.0 * vec2(0.0, 1.0 / resolution.y);

  vec4 u = texture(color_sampler, image_pos + image_step_y);
  vec4 d = texture(color_sampler, image_pos - image_step_y);
  vec4 l= texture(color_sampler, image_pos - image_step_x);
  vec4 r= texture(color_sampler, image_pos + image_step_x);
  vec4 ul= texture(color_sampler, image_pos - image_step_x + image_step_y);
  vec4 ur = texture(color_sampler, image_pos + image_step_x + image_step_y);
  vec4 dl = texture(color_sampler, image_pos - image_step_x - image_step_y);
  vec4 dr = texture(color_sampler, image_pos + image_step_x - image_step_y);

  vec4 vertical_edge = ul + 2.0 * l + dl - ur - 2.0 * r - dr;
  vec4 horizontal_edge = ul + 2.0 * u + ur - dl - 2.0 * d - dr;

  vec4 lc = vec4(0.0, 0.0, 0.0, 1.0);
  vec4 ve = vec4(0.0, 0.0, 0.0, 1.0);
  vec4 he = vec4(0.0, 0.0, 0.0, 1.0);
  vec4 wn = vec4(0.0, 0.0, 0.0, 1.0);

  vec2 crtp = crt_pos(out_position);
  float cm = 1.0 - max(abs(crtp.x), abs(crtp.y)); 

  if (abs(crtp.x) <= 1 && abs(crtp.y) <= 1) {
    lc = cm * texture(color_sampler, image_pos);
    ve = cm * (vertical_edge * vec4(1.0, 0.0, 0.0, 1.0));
    he = cm * (horizontal_edge * vec4(0.0, 1.0, 0.0, 1.0));
    wn = cm * 0.1 * vec4(white_noise(image_pos.xx, time + 0.1),
                     white_noise(image_pos.yy, time + 1.2),
                     white_noise(image_pos.xy, time + 6.3),
                     1.0);
  }
  
  float al = length(lc.xyz);
  
  color = lc + wn + he + ve;
}
