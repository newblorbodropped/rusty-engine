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
                    inputs: &HashSet<ev::VirtualKeyCode, RandomState>,
                    delta_t: f32) {
        match self {
            ModelType::Camera(camera) => {
                let mut  movements : Vec<camera_transformations::CameraMovement> = Vec::new();
                for input in inputs {
                    match input {
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
                if !movements.is_empty() {
                    camera.apply_movement(movements, delta_t);
                }
            }
        }
    }
}

pub struct EventHandler<T> {
    models: Vec<ModelType>,
    pub params: Params,
    inputs: HashSet<ev::VirtualKeyCode, RandomState>,
    phantom: std::marker::PhantomData<T>
}

impl<T> EventHandler<T> {
    pub fn new() -> EventHandler<T> {
        EventHandler {
            models: Vec::new(),
            params: Params { quit: false },
            inputs: HashSet::new(),
            phantom: core::marker::PhantomData
        }
    }

    pub fn add_model(&mut self, model: ModelType) {
        self.models.push(model);
    }
    
    pub fn register_event(&mut self, event: ev::Event<T>) {
        match event {
            ev::Event::WindowEvent { event, .. } => match event {
                ev::WindowEvent::CloseRequested => {
                    self.params.quit = true;
                },
                _ => {}
            },
            ev::Event::DeviceEvent{
		device_id: _,
		event: ev::DeviceEvent::Key(ev::KeyboardInput{
		    virtual_keycode: Some(ev::VirtualKeyCode::Escape),
                    state: ev::ElementState::Pressed,
                    ..
		})
	    }=> {
		self.params.quit = true;
	    },
            ev::Event::DeviceEvent{
		device_id: _,
		event: ev::DeviceEvent::Key(ev::KeyboardInput{
		    virtual_keycode: Some(keycode),
                    state: ev::ElementState::Pressed,
                    ..
		})
	    } => {
		self.inputs.insert(keycode);
	    },
            ev::Event::DeviceEvent{
		device_id: _,
		event: ev::DeviceEvent::Key(ev::KeyboardInput{
		    virtual_keycode: Some(keycode),
                    state: ev::ElementState::Released,
                    ..
		})
	    } => {
		self.inputs.remove(&keycode);
	    },
            _ => {} 
        }
    }

    pub fn modify_models(&mut self, delta_t: f32) {
        for i in 0..self.models.len() {
            self.models[i].apply_inputs(&self.inputs, delta_t);
        }
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
