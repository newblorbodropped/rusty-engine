pub struct Camera {
    pub position: (f32, f32, f32),
    pub front: (f32, f32, f32),
    pub up: (f32, f32, f32),
    pub  right: (f32, f32, f32),
    pub view_aspect_ratio: f32,
    pub fov: f32
}

impl Default for Camera {
    fn default() -> Camera {
        let asp : f32 = 16.0 / 9.0;
        Camera {
            position: (0.0, 0.0, -1.0),
            front: (0.0, 0.0, 1.0),
            up: (0.0, 1.0, 0.0),
            right: (1.0, 0.0, 0.0),
            view_aspect_ratio: asp,
            fov: (1.0 / asp).atan()
        }
    }
}


