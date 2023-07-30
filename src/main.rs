extern crate glium;
extern crate image;

use std::env;
use std::str::FromStr;

use glium::glutin;
use glium::glutin::event as ev;
use glium::glutin::event_loop as evl;

use crate::drawing::mesh::Mesh;
use crate::drawing::mesh;
use crate::drawing::shader_compilation::ShaderProg;
use crate::drawing::texture::Texture;

mod drawing;
mod event_handling;

struct Params {
    full_screen: bool,
    post_pr_id: u16,
    buffer_data: bool,
    scene_file: Option<String>
}

fn parse_params(params: Vec<String>) -> Result<Params, String> {
    let mut full_screen = false;
    let mut post_pr_id : u16 = 0;
    let mut buffer_data = false;
    let mut scene_file : Option<String> = None;

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
            },
            "-b" => {
                buffer_data = true;
            },
            "-s" => {
                match param_iter.next() {
                    None => { return Err("No scene config file given.".to_owned()); },
                    Some(scene) => {
                        scene_file = Some(scene.to_owned())
                    }
                }
            },
            ukwn => {
                let error_message = "Invalid argument given: ".to_owned() + ukwn;
                return Err(error_message);
            }
        }
    }

    Ok(Params {
        full_screen: full_screen,
        post_pr_id: post_pr_id,
        buffer_data: buffer_data,
        scene_file: scene_file
    })
}

fn event_handler_gen<T>(display: glium::Display, scene_file: &str, post_pr_id: u16) ->
impl FnMut(ev::Event<'_, T>, &evl::EventLoopWindowTarget<T>, &mut evl::ControlFlow){
    //load the scene
    let conf = mesh::SceneConfig::load_scene_config(scene_file).unwrap();
    
    //load meshes and extract shader_id, texture_id
    let mut meshes = conf.construct_meshes();
    
    let shader_ids : Vec<u16> = meshes.iter().map(|mesh| mesh.shader_id).collect();
    let texture_ids : Vec<u16> = meshes.iter().map(|mesh| mesh.texture_id).collect();
    
    for mesh in &mut meshes {
        mesh.load_geometry();
        mesh.buffer_unindexed(&display);
    }
    
            //start timer
    let start_time = std::time::Instant::now();
    
    //create an event handler and regster the camera<
    let mut ev_handler = event_handling::EventHandler::new();
    let camera = event_handling::camera_transformations::Camera::default();
    let cam_binding = event_handling::ModelType::Camera(camera);
    ev_handler.add_model(cam_binding);

    //fetch the shaders and textures
    let mut shaders : Vec<ShaderProg> = Vec::new();
    let mut textures : Vec<Texture> = Vec::new();
    
    for shader_id in shader_ids {
        shaders.push(ShaderProg::load_from_file(shader_id, &display));
    }
    
    for texture_id in texture_ids {
        textures.push(Texture::from_file(texture_id, &display));
    }
    
    let shaderpp_prog = ShaderProg::load_from_file_pp(post_pr_id, &display);
    
    move |ev, _, control_flow: &mut glutin::event_loop::ControlFlow| {    
        
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
                drawing::render_meshes(meshes.iter().collect(),
                                       &camera,
                                       &display,
                                       shaders.iter().collect(),
                                       Some(&shaderpp_prog),
                                       textures.iter().collect(),
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
            match par.scene_file {
                Some(scene_file) => {
                    if par.buffer_data {
                        let mut cube = Mesh::new_with_id_shader_tex(1, 1, 1);
                        let mut floor = Mesh::new_with_id_shader_tex(4, 1, 2);
                        
                        cube.load_geometry();
                        floor.load_geometry();
                        
                        match cube.store_to_bin() {
                            Ok(()) => println!("Successfully buffered the cube data!"),
                            Err(err) => err.print_formatted()
                        }
                        
                        match floor.store_to_bin() {
                            Ok(()) => println!("Successfully buffered the floor data!"),
                            Err(err) => err.print_formatted()
                        }
                    } else {          
                        let event_loop = glutin::event_loop::EventLoop::new();
                        let wb = glutin::window::WindowBuilder::new();
                        let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
                        let display = glium::Display::new(wb, cb, &event_loop).unwrap();
                        
                        if par.full_screen {
                            let monitor_handle = display
                                .gl_window()
                                .window()
                                .available_monitors()
                                .next()
                                .unwrap();
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
                        
                        event_loop.run(event_handler_gen(display, &scene_file, par.post_pr_id));
                    }
                },
                None => {
                    println!("No scene configuration given.")
                }
            }
        },
        Err(message) =>  {
            println!("{}", message);
        }
    }
}
