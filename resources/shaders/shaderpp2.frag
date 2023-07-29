#version 140

in vec2 out_position;

uniform sampler2D color_sampler;
uniform float time;
uniform vec2 resolution;

out vec4 color;

const mat4 tmat = mat4(  0.0 / 16.0 ,  8.0 / 16.0 ,  2.0 / 16.0 , 10.0 / 16.0 ,
                        12.0 / 16.0 ,  4.0 / 16.0 , 14.0 / 16.0 ,  6.0 / 16.0 ,
                         3.0 / 16.0 , 11.0 / 16.0 ,  1.0 / 16.0 ,  9.0 / 16.0 ,
                        15.0 / 16.0 ,  7.0 / 16.0 , 13.0 / 16.0 ,  5.0 / 16.0 );

float lum(vec2 pos) {
  vec4 local_color = texture(color_sampler, pos);
  return (local_color.x + local_color.y + local_color.z) / 4.0;
}

float dithering_threshold(vec2 pos) {
  vec2 screen_pos = vec2(pos.x * resolution.x, pos.y * resolution.y);
  vec2 mod_pos = vec2(mod(screen_pos.x, 4.0), mod(screen_pos.y, 4.0));

  int mod_pos_x = int(mod_pos.x);
  int mod_pos_y = int(mod_pos.y);

  return tmat[mod_pos_x][mod_pos_y];
}

void main() {
  vec2 image_pos = out_position * 0.5 + 0.5 ;
  float lum = lum(image_pos);
  float thr = dithering_threshold(image_pos);

  if (lum > thr) {
    color = vec4(1.0, 1.0, 1.0, 1.0);
  } else {
    color = vec4(0.0, 0.0, 0.0, 1.0);
  }
}
