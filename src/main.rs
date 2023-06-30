extern crate glium;

use std::env;
use std::time;

use glium::{glutin, Surface, uniform, index};
use glium::glutin::event as ev;
use glium::glutin::event_loop as evl;

mod model_loading;
mod shader_compilation;
mod camera_transformations;

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
    let (vertex_buf, trans_mat) = model_loading::load_model("resources/collada/first_spaceship.dae", &display);
    
    let program = shader_compilation::create_shader_prog(&display);
    let camera = camera_transformations::Camera::default();
    let start_instant = time::Instant::now(); 
    
    move |ev, _, control_flow| {       
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        let time_passed = time::Instant::now().duration_since(start_instant).as_secs_f32();
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            .. Default::default()
        };
        
        target.draw(&vertex_buf,
                    index::NoIndices(index::PrimitiveType::TrianglesList),
                    &program,
                    &uniform! {
                        camera_pos: [camera.position.0, camera.position.1, camera.position.2],
                        camera_right: [camera.right.0, camera.right.1, camera.right.2],
                        camera_up: [camera.up.0, camera.up.1, camera.up.2],
                        camera_front: [camera.front.0, camera.front.1, camera.front.2],
                        camera_fov: camera.fov,
                        aspect_ratio: camera.view_aspect_ratio,
                        trans_mat: trans_mat,
                        time: time_passed,
                    },
                    &params).unwrap();
        
        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
	    glutin::event::Event::DeviceEvent{
		device_id: _,
		event: glutin::event::DeviceEvent::Key(glutin::event::KeyboardInput{
		    virtual_keycode: Some(glutin::event::VirtualKeyCode::Q),
                    ..
		})
	    } => {
		*control_flow = glutin::event_loop::ControlFlow::Exit;
                return;
	    }
            _ => (),
        }
    }
}

fn main() {
    let params = parse_params(env::args().collect());
    
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    println!("{:#?}", display.get_opengl_version());
    _ = model_loading::load_model("resources/collada/cube.dae", &display);

    if params.full_screen {
        let monitor_handle = display.gl_window().window().available_monitors().next().unwrap();
        let fs = glutin::window::Fullscreen::Borderless(Some(monitor_handle));
        display.gl_window().window().set_fullscreen(Some(fs));
    }
    
    event_loop.run(event_handler_gen(display));
}
