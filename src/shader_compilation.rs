use glium::{Program, Display};

pub fn create_shader_prog(display: &Display) -> Program {
    let vert_src : String = std::fs::read_to_string("resources/shaders/shader.vert").unwrap();
    let frag_src : String = std::fs::read_to_string("resources/shaders/shader.frag").unwrap();

    Program::from_source(display, &vert_src, &frag_src, None).unwrap()
}
