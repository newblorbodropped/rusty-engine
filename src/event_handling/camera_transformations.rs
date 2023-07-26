use std::f32::consts::PI;

mod linalg;

pub struct Camera {
    pub position: [f32; 3],
    pub front: [f32; 3],
    pub up: [f32; 3],
    pub  right: [f32; 3],
    pub view_aspect_ratio: f32,
    pub fov: f32,
    mov_speed: f32,
    rot_speed: f32
}

#[derive(Debug)]
pub enum CameraMovement {
    MovLeft,
    MovRight,
    MovForward,
    MovBackward,
    MovUp,
    MovDown,
    RotLeft,
    RotRight,
    RotUp,
    RotDown
}

impl Default for Camera {
    fn default() -> Camera {
        let asp : f32 = 16.0 / 9.0;
        Camera {
            position: [0.0, 0.0, -1.0],
            front: [0.0, 0.0, 1.0],
            up: [0.0, 1.0, 0.0],
            right: [1.0, 0.0, 0.0],
            view_aspect_ratio: asp,
            fov: 1.0,
            mov_speed: 0.01,
            rot_speed: 0.2
        }
    }
}

impl Camera {
    pub fn apply_movement(&mut self, actions: Vec<CameraMovement>) {
        let mut mov_dir : [f32; 3] = [0.0, 0.0, 0.0];
        for action in actions.iter() {
            match action {
                CameraMovement::MovLeft => {
                    mov_dir = [mov_dir[0] - self.right[0],
                               mov_dir[1] - self.right[1],
                               mov_dir[2] - self.right[2]];
                },
                CameraMovement::MovRight => {
                    mov_dir = [mov_dir[0] + self.right[0],
                               mov_dir[1] + self.right[1],
                               mov_dir[2] + self.right[2]];
                },
                CameraMovement::MovForward => {
                    mov_dir = [mov_dir[0] + self.front[0],
                               mov_dir[1] + self.front[1],
                               mov_dir[2] + self.front[2]];
                },
                CameraMovement::MovBackward => {
                    mov_dir = [mov_dir[0] - self.front[0],
                               mov_dir[1] - self.front[1],
                               mov_dir[2] - self.front[2]];
                },
                CameraMovement::MovUp => {
                    mov_dir = [mov_dir[0] + self.up[0],
                               mov_dir[1] + self.up[1],
                               mov_dir[2] + self.up[2]];
                },
                CameraMovement::MovDown => {
                    mov_dir = [mov_dir[0] - self.up[0],
                               mov_dir[1] - self.up[1],
                               mov_dir[2] - self.up[2]];
                },
                CameraMovement::RotLeft => {
                    let rotmat : [[f32; 3]; 3] =
                        linalg::rotmat([0.0, 1.0, 0.0], self.rot_speed * (PI / 180.0));
                    self.up = linalg::matmulvec3(rotmat, self.up);
                    self.right = linalg::matmulvec3(rotmat, self.right);
                    self.front = linalg::matmulvec3(rotmat, self.front);
                },
                CameraMovement::RotRight => {
                    let rotmat : [[f32; 3]; 3] =
                        linalg::rotmat([0.0, 1.0, 0.0], (-1.0) * self.rot_speed * (PI / 180.0));
                    self.up = linalg::matmulvec3(rotmat, self.up);
                    self.right = linalg::matmulvec3(rotmat, self.right);
                    self.front = linalg::matmulvec3(rotmat, self.front);
                },
                CameraMovement::RotUp => {
                    let rotmat : [[f32; 3]; 3] =
                        linalg::rotmat(self.right, self.rot_speed * (PI / 180.0));
                    self.up = linalg::matmulvec3(rotmat, self.up);
                    self.right = linalg::matmulvec3(rotmat, self.right);
                    self.front = linalg::matmulvec3(rotmat, self.front);
                },
                CameraMovement::RotDown => {
                    let rotmat : [[f32; 3]; 3] =
                        linalg::rotmat(self.right, (-1.0) * self.rot_speed * (PI / 180.0));
                    self.up = linalg::matmulvec3(rotmat, self.up);
                    self.right = linalg::matmulvec3(rotmat, self.right);
                    self.front = linalg::matmulvec3(rotmat, self.front);
                }
            }
        }

        mov_dir = {
            let norm = (mov_dir[0] * mov_dir[0] + mov_dir[1] * mov_dir[1] + mov_dir[2] * mov_dir[2]).sqrt();
            if norm > 0.01 {
                [mov_dir[0] / norm, mov_dir[1]/ norm, mov_dir[2] / norm]
            } else {
                [0.0, 0.0, 0.0]
            }
        };
        
        self.position = [self.position[0] + self.mov_speed * mov_dir[0],
                         self.position[1] + self.mov_speed * mov_dir[1],
                         self.position[2] + self.mov_speed * mov_dir[2]];
    }
}

