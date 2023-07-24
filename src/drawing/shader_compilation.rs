use glium::{Program, Display};

pub struct ShaderProg {
    pub id: u16,
    pub prog: Program
}

impl ShaderProg {
    pub fn load_from_file(id: u16, display: &Display) -> ShaderProg {
        let mut pathstr_vert = std::string::String::new();
        pathstr_vert.push_str("./resources/shaders/shader");
        pathstr_vert.push_str(id.to_string().as_str());
        pathstr_vert.push_str(".vert");
        let path_vert = std::path::Path::new(&pathstr_vert);

        let mut pathstr_frag = std::string::String::new();
        pathstr_frag.push_str("./resources/shaders/shader");
        pathstr_frag.push_str(id.to_string().as_str());
        pathstr_frag.push_str(".frag");
        let path_frag = std::path::Path::new(&pathstr_frag);

        let vert_src : String = std::fs::read_to_string(path_vert).unwrap();
        let frag_src : String = std::fs::read_to_string(path_frag).unwrap();

        let program = Program::from_source(display, &vert_src, &frag_src, None).unwrap();
        
        ShaderProg {
            id: id,
            prog: program
        }
    }

    pub fn load_from_file_pp(id: u16, display: &Display) -> ShaderProg {
        let mut pathstr_vert = std::string::String::new();
        pathstr_vert.push_str("./resources/shaders/shaderpp.vert");
        let path_vert = std::path::Path::new(&pathstr_vert);

        let mut pathstr_frag = std::string::String::new();
        pathstr_frag.push_str("./resources/shaders/shaderpp");
        pathstr_frag.push_str(id.to_string().as_str());
        pathstr_frag.push_str(".frag");
        let path_frag = std::path::Path::new(&pathstr_frag);
        
        let vert_src : String = std::fs::read_to_string(path_vert).unwrap();
        let frag_src : String = std::fs::read_to_string(path_frag).unwrap();

        let program = Program::from_source(display, &vert_src, &frag_src, None).unwrap();
        
        ShaderProg {
            id: id,
            prog: program
        }
    }

    pub fn get_prog(&self) -> &Program {
        &self.prog
    }
}
