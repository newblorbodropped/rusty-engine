#version 140
in vec3 position;
in vec3 normal;

uniform vec3 camera_pos;
uniform vec3 camera_right;
uniform vec3 camera_up;
uniform vec3 camera_front;
uniform float camera_fov;
uniform float aspect_ratio;
uniform float time;

const float zrange = 12.0;
const vec3 obj_pos = vec3(0.0, 0.0, 3.0);

out vec3 out_normal;
out vec3 out_position;

vec3 centering_vec() {
  return (camera_pos + ((tan(camera_fov * 0.5) * (1 / aspect_ratio) * 0.5) * camera_front));
}

mat3 align_matrix() {
  return transpose(mat3(camera_right, camera_up, camera_front));
}

mat4 projection_matrix() {
  return mat4(
              camera_pos.z,
              0.0,
              0.0,
              0.0,
              0.0,
              aspect_ratio * camera_pos.z,
              0.0,
              0.0,
              (-1.0) * camera_pos.x,
              (-1.0) * aspect_ratio * camera_pos.y,
              1.0 / zrange,
              1.0,
              0.0,
              0.0,
              (2.0 * tan(1.0 / 2.0 * aspect_ratio)) / zrange,
              (-1.0) * camera_pos.z
              );
}

mat3 rot_mat(vec3 v, float t) {
    vec3 vn = normalize(v);
    float sinth = sin(0.5 * t);
    float costh = cos(0.5 * t);
    return mat3(
            1 - 2*(vn.y * vn.y * sinth * sinth + vn.z * vn.z * sinth * sinth),
            2*(vn.x * vn.y * sinth * sinth - vn.z * sinth * costh),
            2*(vn.x * vn.z * sinth * sinth + vn.y * sinth * costh),
            2*(vn.x * vn.y * sinth * sinth + vn.z * sinth * costh),
            1 - 2*(vn.x * vn.x * sinth * sinth + vn.z * vn.z * sinth * sinth),
            2*(vn.y * vn.z * sinth * sinth - vn.x * sinth * costh),
            2*(vn.x * vn.z * sinth * sinth - vn.y * sinth * costh),
            2*(vn.y * vn.z * sinth * sinth + vn.x * sinth * costh),
            1 - 2*(vn.x * vn.x * sinth * sinth + vn.y * vn.y * sinth * sinth)
        );
}


void main() {
  mat3 rotation = rot_mat(vec3(0, 1, 0), time);
  
  vec3 aligned_pos = (rotation * position) + obj_pos + (sin(time) * vec3(0.0, 2.0, 0.0));

  float lambda = 1 / (2 * aligned_pos.z + 1);
  gl_Position.x = aligned_pos.x * lambda;
  gl_Position.y = (16.0 / 9.0) * aligned_pos.y * lambda;
  gl_Position.z = 0.125 * aligned_pos.z - 1;
  gl_Position.w = 1.0;
    
  //vec4 new_pos = projection_matrix() * vec4(position, 1.0);
  out_position = gl_Position.xyz;
  out_normal = rotation * normal;
  //gl_Position = new_pos;
}
