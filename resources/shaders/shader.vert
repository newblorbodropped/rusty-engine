#version 140
in vec3 position;
in vec3 normal;
in vec2 tex_coords;

uniform vec3 camera_pos;
uniform vec3 camera_right;
uniform vec3 camera_up;
uniform vec3 camera_front;
uniform float camera_fov;
uniform float aspect_ratio;
uniform mat4 trans_mat;
uniform float time;


const float z_near = 0.5;

const vec3 obj_pos = vec3(0.0, 0.0, 10.0);

out vec3 out_normal;
out vec3 out_position;
out vec2 out_tex_coords;

vec3 centering_vec() {
  return (camera_pos + ((tan(camera_fov * 0.5) * (1 / aspect_ratio) * 0.5) * camera_front));
}

mat3 align_matrix() {
  return transpose(mat3(camera_right, camera_up, camera_front));
}

mat4 projection_matrix() {
  float t = tan(0.5 * camera_fov);
  float n = z_near;
  float a = aspect_ratio;
  
  return mat4(
              1.0 / (t * a) , 0.0 , 0.0 , 0.0 ,
              0.0 , 1.0 / t , 0.0 , 0.0 ,
              0.0 , 0.0 , 1.0 , 1.0,
              0.0 , 0.0 , (-1.0) * n , 0.0
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
  vec4 p = trans_mat * vec4(position, 1.0);
  p.xyz = (1.0 / p.w) * p.xyz;
  
  vec3 aligned_pos = (rotation * p.xyz) + obj_pos + (sin(time) * vec3(0.0, 2.0, 0.0));
  
  gl_Position = projection_matrix() * vec4(aligned_pos, 1.0);
  
  //vec4 new_pos = projection_matrix() * vec4(position, 1.0);
  out_position = gl_Position.xyz;
  out_normal = rotation * (trans_mat * vec4(normal, 1.0)).xyz;
  out_tex_coords = tex_coords;
  //gl_Position = new_pos;
}
