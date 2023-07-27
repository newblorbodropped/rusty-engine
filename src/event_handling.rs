use glium::glutin::event as ev;

use std::collections::HashSet;
use std::collections::hash_map::RandomState;

pub mod camera_transformations;

pub struct Params {
    pub quit: bool
}

pub enum ModelType {
    Camera(camera_transformations::Camera)
}

impl ModelType {
    fn apply_inputs(&mut self,
                    keypresses: &HashSet<ev::VirtualKeyCode, RandomState>,
                    mouse_move: &Option<(f64, f64)>) {
        match self {
            ModelType::Camera(camera) => {
                let mut  movements : Vec<camera_transformations::CameraMovement> = Vec::new();
                for keypress in keypresses {
                    match keypress {
                        ev::VirtualKeyCode::W => {
                            movements.push(camera_transformations::CameraMovement::MovForward);
                        },
                        ev::VirtualKeyCode::S => {
                            movements.push(camera_transformations::CameraMovement::MovBackward);
                        },
                        ev::VirtualKeyCode::A => {
                            movements.push(camera_transformations::CameraMovement::MovLeft);
                        },
                        ev::VirtualKeyCode::D => {
                            movements.push(camera_transformations::CameraMovement::MovRight);
                        },
                        ev::VirtualKeyCode::Space => {
                            movements.push(camera_transformations::CameraMovement::MovUp);
                        },
                        ev::VirtualKeyCode::LControl => {
                            movements.push(camera_transformations::CameraMovement::MovDown);
                        },
                        ev::VirtualKeyCode::Up => {
                            movements.push(camera_transformations::CameraMovement::RotUp);
                        },
                        ev::VirtualKeyCode::Down => {
                            movements.push(camera_transformations::CameraMovement::RotDown);
                        },
                        ev::VirtualKeyCode::Left => {
                            movements.push(camera_transformations::CameraMovement::RotLeft);
                        },
                        ev::VirtualKeyCode::Right => {
                            movements.push(camera_transformations::CameraMovement::RotRight);
                        },
                        _ => {}
                    }
                }

                match *mouse_move {
                    Some((dx, dy)) => {
                        movements.push(camera_transformations::CameraMovement::RotateDir(dx, dy));
                        }
                    None => {}
                }
                
                if !movements.is_empty() {
                    camera.apply_movement(movements);
                }
            }
        }
    }
}

pub struct EventHandler {
    models: Vec<ModelType>,
    pub params: Params,
    keypresses: HashSet<ev::VirtualKeyCode, RandomState>,
    mouse_move: Option<(f64, f64)> 
}

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler {
            models: Vec::new(),
            params: Params { quit: false },
            keypresses: HashSet::new(),
            mouse_move: None
        }
    }

    pub fn add_model(&mut self, model: ModelType) {
        self.models.push(model);
    }
    
    pub fn register_window_event(&mut self, event: ev::WindowEvent) {
        match event {
            ev::WindowEvent::CloseRequested => {
                 self.params.quit = true;
            }
            ev::WindowEvent::KeyboardInput {
                device_id: _,
                input: keyboard_input,
                ..
            } => {
                match keyboard_input.virtual_keycode {
                    Some(keycode) => {
                        if keycode == ev::VirtualKeyCode::Escape {
                            self.params.quit = true;
                        }
                        match keyboard_input.state {
                            ev::ElementState::Pressed => self.keypresses.insert(keycode),
                            ev::ElementState::Released => self.keypresses.remove(&keycode),
                        };
                    },
                    None => {}
                }
            },
            _ => {}
        }
    }

    pub fn register_device_event(&mut self, event: ev::DeviceEvent) {
        match event {
            ev::DeviceEvent::MouseMotion{delta: (dx, dy)} => {
                self.mouse_move = Some((dx, dy));
            },
            _ => {}
        }
    }

    pub fn modify_models(&mut self) {
        for i in 0..self.models.len() {
            self.models[i].apply_inputs(&self.keypresses, &self.mouse_move);
        }
        self.mouse_move = None;
    }

    pub fn get_camera(&self) -> Option<&camera_transformations::Camera> {
        for model in &self.models {
            match model {
                ModelType::Camera(camera) => { return Some(&camera); }
            }
        }
        None
    }
}
