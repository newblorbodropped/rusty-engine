extern crate glium;
extern crate image;

use std::env;
use std::str::FromStr;

use glium::glutin;
use glium::glutin::event as ev;
use glium::glutin::event_loop as evl;

use crate::drawing::mesh::Mesh;
use crate::drawing::shader_compilation::ShaderProg;
use crate::drawing::texture::Texture;

mod drawing;
mod event_handling;

struct Params {
    full_screen: bool,
    post_pr_id: u16
}

fn parse_params(params: Vec<String>) -> Result<Params, String> {
    let mut full_screen = false;
    let mut post_pr_id : u16 = 0;

    let mut param_iter = params.iter();

    //throwing away the program name itself
    let _ = param_iter.next(); 
    
    while let Some(param) = param_iter.next() {
        match param.as_str() {
            "-f" => {
                full_screen = true;
            },
            "-p" => {
                match param_iter.next() {
                    None => { return Err("No shader for postprocessing given.".to_owned()); },
                    Some(opt) => {
                        match u16::from_str(opt.as_str()) {
                            Ok(id) => { post_pr_id = id; },
                            Err(_) => { return Err("Invalid shader for postprocessing given.".to_owned()); }
                        }
                    }
                }
            }
            ukwn => {
                let error_message = "Invalid argument given: ".to_owned() + ukwn;
                return Err(error_message);
            }
        }
    }

    Ok(Params {
        full_screen: full_screen,
        post_pr_id: post_pr_id
    })
}

fn event_handler_gen<T>(display: glium::Display, post_pr_id: u16) ->
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
    let shaderpp_prog = ShaderProg::load_from_file_pp(post_pr_id, &display);

    //create a camera
    let camera = event_handling::camera_transformations::Camera::default();

    //start timing
    let start_time = std::time::Instant::now();
    
    //create an event handler and regster the camera<
    let mut ev_handler = event_handling::EventHandler::new();
    let cam_binding = event_handling::ModelType::Camera(camera);
    ev_handler.add_model(cam_binding);

    //load all the model textures
    let cube_tex = Texture::from_file(1, &display);
    let floor_tex = Texture::from_file(2, &display);
    
    move |ev, _, control_flow| {    
        if ev_handler.params.quit {
            *control_flow = glutin::event_loop::ControlFlow::Exit;
            return;
        }

        match ev {
            ev::Event::RedrawEventsCleared => {
                display.gl_window().window().request_redraw();
            }
            ev::Event::RedrawRequested(_) => {
                let this_frame = std::time::Instant::now();
                let time = this_frame.duration_since(start_time).as_secs_f32();
                ev_handler.modify_models();
                let camera = ev_handler.get_camera().unwrap();
                drawing::render_meshes(vec![&cube, &floor],
                                       &camera,
                                       &display,
                                       vec![&shader_prog],
                                       Some(&shaderpp_prog),
                                       vec![&cube_tex, &floor_tex],
                                       time);
            },
            ev::Event::WindowEvent { event, .. } => {
                ev_handler.register_window_event(event);
            },
            ev::Event::DeviceEvent { event, .. } => {
                ev_handler.register_device_event(event);
            }
            _ => {}
        }
    }
}

fn main() {
    let params = parse_params(env::args().collect());

    match params {
        Ok(par) => {
            let event_loop = glutin::event_loop::EventLoop::new();
            let wb = glutin::window::WindowBuilder::new();
            let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
            let display = glium::Display::new(wb, cb, &event_loop).unwrap();

            if par.full_screen {
                let monitor_handle = display.gl_window().window().available_monitors().next().unwrap();
                let fs = glutin::window::Fullscreen::Borderless(Some(monitor_handle));
                display.gl_window().window().set_fullscreen(Some(fs));
                display.gl_window().window().set_cursor_visible(false);
                display.gl_window().window()
                    .set_cursor_grab(glutin::window::CursorGrabMode::Confined)
                    .or_else(|_e| {
                        display.gl_window().window()
                            .set_cursor_grab(glutin::window::CursorGrabMode::Locked)
                    })
                    .unwrap();
            }
            
            event_loop.run(event_handler_gen(display, par.post_pr_id));
        },
        Err(message) =>  {
            println!("{}", message);
        }
    }
}
