use glium::backend::glutin::Display;
use glium::vertex::VertexBuffer;

use model_loading::{ Position, Normal, TextureCoordinates , Vertex };
use model_loading::collada_parsing::Parser;

pub mod model_loading;

pub struct Mesh {
    pub id: u16,
    pub positions: Option<Box<[Position]>>,
    pub normals: Option<Box<[Normal]>>,
    pub tex_coords: Option<Box<[TextureCoordinates]>>,
    pub indices: Option<Box<[u16]>>,
    pub vertex_buf: Option<VertexBuffer<Vertex>>,
    pub transform_mat: [[f32; 4]; 4],
    pub offset: (f32, f32, f32),
    pub scale: f32,
    pub shader_id: u16,
    pub texture_id: u16
}

impl Mesh {
    #[allow(dead_code)]
    pub fn new() -> Mesh {
        let mat : [[f32; 4]; 4] = [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0]
        ];

        Mesh {
            id: 0,
            positions: None,
            normals: None,
            tex_coords: None,
            indices: None,
            vertex_buf: None,
            transform_mat: mat,
            offset: (0.0, 0.0, 0.0),
            scale: 1.0,
            shader_id: 0,
            texture_id: 0
        }
    }

    #[allow(dead_code)]
    pub fn new_with_id(id: u16) -> Mesh {
        let mat : [[f32; 4]; 4] = [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0]
        ];
        
        Mesh {
            id: id,
            positions: None,
            normals: None,
            tex_coords: None,
            indices: None,
            vertex_buf: None,
            transform_mat: mat,
            offset: (0.0, 0.0, 0.0),
            scale: 1.0,
            shader_id: 0,
            texture_id: 0
        }
    }

    #[allow(dead_code)]
    pub fn new_with_id_shader(id: u16, shader_id: u16) -> Mesh{
        let mat : [[f32; 4]; 4] = [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0]
        ];
        
        Mesh {
            id: id,
            positions: None,
            normals: None,
            tex_coords: None,
            indices: None,
            vertex_buf: None,
            transform_mat: mat,
            offset: (0.0, 0.0, 0.0),
            scale: 1.0,
            shader_id: shader_id,
            texture_id: 0
        }
    }

    pub fn new_with_id_shader_tex(id: u16, shader_id: u16, texture_id: u16) -> Mesh {
        let mat : [[f32; 4]; 4] = [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0]
        ];
        
        Mesh {
            id: id,
            positions: None,
            normals: None,
            tex_coords: None,
            indices: None,
            vertex_buf: None,
            transform_mat: mat,
            offset: (0.0, 0.0, 0.0),
            scale: 1.0,
            shader_id: shader_id,
            texture_id: texture_id
        }
    }

    #[allow(dead_code)]
    pub fn set_id(&mut self, id: u16) {
        self.id = id;
    }

    pub fn set_offset(&mut self, offset: (f32, f32, f32)) {
        self.offset = offset;
    }

    #[allow(dead_code)]
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn load_geometry(&mut self) {
        let mut pathstr = std::string::String::new();
        pathstr.push_str("./resources/collada/model");
        pathstr.push_str(self.id.to_string().as_str());
        pathstr.push_str(".dae");
        let path = std::path::Path::new(&pathstr); 
        let source = std::fs::read_to_string(path).unwrap();
        let (_, collada_model) = model_loading::collada_parsing::collada_p().parse(&source[..]).unwrap();

        let maybe_pos_vec = model_loading::extract_positions(&collada_model);
        let maybe_norm_vec = model_loading::extract_normals(&collada_model);
        let maybe_tex_coords_vec = model_loading::extract_texture_coordinates(&collada_model);
        let maybe_indices_vec = model_loading::extract_indices(&collada_model);
        let maybe_trans_mat = model_loading::extract_transform_mat(&collada_model);

        self.positions = maybe_pos_vec.map(|vec| vec.into_boxed_slice());
        self.normals = maybe_norm_vec.map(|vec| vec.into_boxed_slice());
        self.tex_coords = maybe_tex_coords_vec.map(|vec| vec.into_boxed_slice());
        self.indices = maybe_indices_vec.map(|vec| vec.into_boxed_slice());

        if !maybe_trans_mat.is_none() {
            self.transform_mat = maybe_trans_mat.unwrap();
        }
    }

    pub fn buffer_unindexed(&mut self, display: &Display) { 
        let mut res : Vec<Vertex> = Vec::new();

        match self.positions {
            Some(_) => {},
            None => { return; }
        }

        match self.normals {
            Some(_) => {},
            None => { return; }
        }

        match self.tex_coords {
            Some(_) => {},
            None => { return; }
        }

        match self.indices {
            Some(_) => {},
            None => { return; }
        }
        
        let positions = self.positions.clone().unwrap();
        let normals = self.normals.clone().unwrap();
        let tex_coords = self.tex_coords.clone().unwrap();
        let indices = self.indices.clone().unwrap();

        let indx_len = indices.len();

        if indx_len % 3 != 0 {
            return;
        }
        
        let mut curr_indx : usize = 0;

        while curr_indx < indx_len {
            let mut vert : Vertex = Vertex::default();
            
            let pos_index : usize = indices[curr_indx] as usize;
            vert.position = positions[pos_index].position;

            curr_indx += 1;

            let norm_index : usize = indices[curr_indx] as usize;
            vert.normal = normals[norm_index].normal;

            curr_indx += 1;

            let tex_coord_index : usize = indices[curr_indx] as usize;
            vert.tex_coords = tex_coords[tex_coord_index].coordinates;

            curr_indx += 1;

            res.push(vert);
        }

        self.vertex_buf = Some(VertexBuffer::new(display, res.as_slice()).unwrap());
    }
}
