extern crate glium;
extern crate image;

use std::env;
use std::time;

use glium::{glutin, Surface, uniform, index};
use glium::glutin::event as ev;
use glium::glutin::event_loop as evl;

use crate::drawing::mesh::Mesh;
use crate::drawing::shader_compilation::ShaderProg;
use crate::drawing::texture::Texture;

mod drawing;
mod event_handling;

struct Params {
    full_screen: bool
}

fn parse_params(params: Vec<String>) -> Params {
    if params.is_empty() {
        Params{full_screen: false}
    } else {
        let mut params = params.iter();
    params.next();
        match params.next() {
            Some(_) => Params{full_screen: true},
            _ => Params{full_screen: false}
        }
    }
}

fn event_handler_gen<T>(display: glium::Display) ->
impl FnMut(ev::Event<'_, T>, &evl::EventLoopWindowTarget<T>, &mut evl::ControlFlow){
    //load the models
    let mut cube = Mesh::new_with_id_shader_tex(1, 1, 1);
    let mut floor = Mesh::new_with_id_shader_tex(4, 1, 2);
    
    cube.load_geometry();
    cube.buffer_unindexed(&display);

    floor.set_offset((0.0 as f32, -1.0 as f32, 0.0 as f32));
    floor.load_geometry();
    floor.buffer_unindexed(&display);

    //load the shader programs
    let shader_prog = ShaderProg::load_from_file(1, &display);
    let shaderpp_prog = ShaderProg::load_from_file_pp(0, &display);

    //create a camera
    let camera = event_handling::camera_transformations::Camera::default();

    //create an event handler and regster the camera<
    let mut ev_handler = event_handling::EventHandler::new();
    let cam_binding = event_handling::ModelType::Camera(camera);
    ev_handler.add_model(cam_binding);

    //start the clock
    let start_instant = time::Instant::now();
    let mut last_frame = time::Instant::now();

    //load all the model textures
    let cube_tex = Texture::from_file(1, &display);
    let floor_tex = Texture::from_file(2, &display);
    
    move |ev, _, control_flow| {
        let time_passed = time::Instant::now().duration_since(start_instant).as_secs_f32();
        let time_since_last_frame = time::Instant::now().duration_since(last_frame).as_secs_f32();
        println!("{}", time_since_last_frame);
        last_frame = time::Instant::now();
        
        let camera = ev_handler.get_camera().unwrap();

        drawing::render_meshes(vec![&cube, &floor],
                               &display,
                               &camera,
                               vec![&shader_prog],
                               Some(&shaderpp_prog),
                               vec![&cube_tex, &floor_tex]);

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        
        ev_handler.register_event(ev);
        ev_handler.modify_models();
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        if ev_handler.params.quit {
            *control_flow = glutin::event_loop::ControlFlow::Exit;
            return;
        }
    }
}

fn main() {
    let params = parse_params(env::args().collect());
    
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    if params.full_screen {
        let monitor_handle = display.gl_window().window().available_monitors().next().unwrap();
        let fs = glutin::window::Fullscreen::Borderless(Some(monitor_handle));
        display.gl_window().window().set_fullscreen(Some(fs));
    }
    
    event_loop.run(event_handler_gen(display));
}
