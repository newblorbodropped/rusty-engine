use glium::{VertexBuffer, implement_vertex};
use crate::drawing::mesh::model_loading::collada_parsing::{Collada, collada_p, Parser, TagParameter};

pub mod collada_parsing;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub struct Vertex {
    pub position: (f32, f32, f32),
    pub normal: (f32, f32, f32),
    pub tex_coords: (f32, f32)
}

impl Default for Vertex {
    fn default() -> Vertex {
        Vertex {
            position: (0.0, 0.0, 0.0),
            normal: (0.0, 0.0, 0.0),
            tex_coords: (0.0, 0.0),
        }
    }
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub struct Position {
    pub position: (f32, f32, f32)
}

impl Default for Position {
    fn default() -> Position {
        Position{
            position: (0.0, 0.0, 0.0)
        }
    }
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub struct Normal {
    pub normal: (f32, f32, f32)
}

impl Default for Normal {
    fn default() -> Normal {
        Normal{
            normal: (0.0, 0.0, 0.0)
        }
    }
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub struct TextureCoordinates {
    pub coordinates: (f32, f32)
}

impl Default for TextureCoordinates {
    fn default() -> TextureCoordinates {
        TextureCoordinates{
            coordinates: (0.0, 0.0)
        }
    }
}

implement_vertex!(Vertex, position, normal, tex_coords);
implement_vertex!(Position, position);
implement_vertex!(Normal, normal);
implement_vertex!(TextureCoordinates, coordinates);

fn find_tag_name<'a>(source: &'a Collada<'a>, name: &'a str) -> Option<&'a Collada<'a>> {
    match source {
        Collada::ColladaTag(_, _, cont) => {
            let contents = (*cont).iter();
            let mut res: Option<&Collada> = None;
            for item in contents {
                match item {
                    Collada::ColladaTag(tag_name, _, _) => {
                        if tag_name.eq(&name) {
                            res = Some(&item);
                        }
                    },
                    _ => {}
                }
            }
            res
        },
        _ => None
    }
}

fn find_floats<'a>(source: &'a Collada<'a>) -> Option<Vec<f32>> {  
    match source {
        Collada::ColladaTag(_ ,_ , cont) => {
            match (*cont).iter().next() {
                Some(Collada::ColladaFloats(vec)) => Some(vec.to_vec()),
                _ => None
            }
        },
        _ => None
    }    
}

pub fn group_to_positions(mut floats: Vec<f32>) -> Vec<Position> {
    let mut res: Vec<Position> = Vec::new();
    while floats.len() >= 3 {
        let (coords, rest) = floats.split_at(3);
        res.push(Position{ position: (coords[0], coords[1], coords[2]) });
        floats = Vec::from(rest);
    }
    res
}

pub fn group_to_normals(mut floats: Vec<f32>) -> Vec<Normal> {
    let mut res: Vec<Normal> = Vec::new();
    while floats.len() >= 3 {
        let (coords, rest) = floats.split_at(3);
        res.push(Normal{ normal: (coords[0], coords[1], coords[2]) });
        floats = Vec::from(rest);
    }
    res
}

pub fn group_to_tex_coords(mut floats: Vec<f32>) -> Vec<TextureCoordinates> {
    let mut res: Vec<TextureCoordinates> = Vec::new();
    while floats.len() >= 2 {
        let (coords, rest) = floats.split_at(2);
        res.push(TextureCoordinates{ coordinates: (coords[0], coords[1]) });
        floats = Vec::from(rest);
    }
    res
}

pub fn to_indices(floats: Vec<f32>) -> Vec<u16> {
    let mut res : Vec<u16> = Vec::new();
    for x in floats.iter() {
        res.push(*x as u16);
    }
    res
}

pub fn to_matrix(floats: Vec<f32>) -> Option<[[f32; 4]; 4]> {
    unsafe {
        if floats.len() != 16 {
            None 
        } else {
            Some(
                [
                    [*floats.get_unchecked(0), *floats.get_unchecked(4),
                     *floats.get_unchecked(8), *floats.get_unchecked(12)],
                    [*floats.get_unchecked(1), *floats.get_unchecked(5),
                     *floats.get_unchecked(9), *floats.get_unchecked(13)],
                    [*floats.get_unchecked(2), *floats.get_unchecked(6),
                     *floats.get_unchecked(10), *floats.get_unchecked(14)],
                    [*floats.get_unchecked(3), *floats.get_unchecked(7),
                     *floats.get_unchecked(11), *floats.get_unchecked(15)]
                ]
            )
    }
    }
}

pub fn extract_positions<'a>(source: &'a Collada<'a>) -> Option<Vec<Position>> {
    match source {
        Collada::ColladaHeader(b) => {
            match find_tag_name(&b,"library_geometries") {
                Some(x1) => {
                    match find_tag_name(x1, "geometry") {
                        Some(x2) => {
                            match find_tag_name(x2, "mesh") {
                                Some(x3) => {
                                    match x3 {
                                        Collada::ColladaTag(_, _, cont) => {
                                            let contents = (*cont).iter();
                                            let mut res: Option<&Collada> = None;
                                            for item in contents {
                                                match item {
                                                    Collada::ColladaTag(_, params, _) => {
                                                        if let Some(TagParameter::ParameterString(name, val)) =
                                                            params.iter().next() {
                                                                if name.eq(&"id") &&
                                                                    val.contains("position") {
                                                                        res = Some(item);
                                                                }
                                                            }
                                                    },
                                                    _ => {}
                                                }
                                            }
                                            match res {
                                                None => None,
                                                Some(x) => {
                                                    match find_tag_name(x, "float_array") {  
                                                        Some(floats_tag) => {
                                                            match find_floats(floats_tag) {
                                                                Some(vec) => Some(group_to_positions(vec)),
                                                                None => None
                                                            }
                                                        },
                                                        None => None
                                                    }
                                                }
                                            }
                                        }
                                        _ => None
                                    }
                                },
                                None => None
                            }
                        },
                        None => None
                    }
                },
                None => None
            }
        },
        _ => None
    }
}

pub fn extract_normals<'a>(source: &'a Collada<'a>) -> Option<Vec<Normal>> {
    match source {
        Collada::ColladaHeader(b) => {
            match find_tag_name(&b,"library_geometries") {
                Some(x1) => {
                    match find_tag_name(x1, "geometry") {
                        Some(x2) => {
                            match find_tag_name(x2, "mesh") {
                                Some(x3) => {
                                    match x3 {
                                        Collada::ColladaTag(_, _, cont) => {
                                            let contents = (*cont).iter();
                                            let mut res: Option<&Collada> = None;
                                            for item in contents {
                                                match item {
                                                    Collada::ColladaTag(_, params, _) => {
                                                        if let Some(TagParameter::ParameterString(name, val)) =
                                                            params.iter().next() {
                                                                if name.eq(&"id") &&
                                                                    val.contains("normal") {
                                                                        res = Some(item);
                                                                }
                                                            }
                                                    },
                                                    _ => {}
                                                }
                                            }
                                            match res {
                                                None => None,
                                                Some(x) => {
                                                    match find_tag_name(x, "float_array") {  
                                                        Some(floats_tag) => {
                                                            match find_floats(floats_tag) {
                                                                Some(vec) => Some(group_to_normals(vec)),
                                                                None => None
                                                            }
                                                        },
                                                        None => None
                                                    }
                                                }
                                            }
                                        }
                                        _ => None
                                    }
                                },
                                None => None
                            }
                        },
                        None => None
                    }
                },
                None => None
            }
        },
        _ => None
    }
}

pub fn extract_texture_coordinates<'a>(source: &'a Collada<'a>) -> Option<Vec<TextureCoordinates>> {
    match source {
        Collada::ColladaHeader(b) => {
            match find_tag_name(&b,"library_geometries") {
                Some(x1) => {
                    match find_tag_name(x1, "geometry") {
                        Some(x2) => {
                            match find_tag_name(x2, "mesh") {
                                Some(x3) => {
                                    match x3 {
                                        Collada::ColladaTag(_, _, cont) => {
                                            let contents = (*cont).iter();
                                            let mut res: Option<&Collada> = None;
                                            for item in contents {
                                                match item {
                                                    Collada::ColladaTag(_, params, _) => {
                                                        if let Some(TagParameter::ParameterString(name, val)) =
                                                            params.iter().next() {
                                                                if name.eq(&"id") &&
                                                                    val.contains("map") {
                                                                        res = Some(item);
                                                                }
                                                            }
                                                    },
                                                    _ => {}
                                                }
                                            }
                                            match res {
                                                None => None,
                                                Some(x) => {
                                                    match find_tag_name(x, "float_array") {  
                                                        Some(floats_tag) => {
                                                            match find_floats(floats_tag) {
                                                                Some(vec) => Some(group_to_tex_coords(vec)),
                                                                None => None
                                                            }
                                                        },
                                                        None => None
                                                    }
                                                }
                                            }
                                        }
                                        _ => None
                                    }
                                },
                                None => None
                            }
                        },
                        None => None
                    }
                },
                None => None
            }
        },
        _ => None
    }
}

pub fn extract_indices<'a>(source: &'a Collada<'a>) -> Option<Vec<u16>> {
    match source {
        Collada::ColladaHeader(b) => {
            match find_tag_name(b, "library_geometries") {
                Some(x1) => {
                    match find_tag_name(x1, "geometry") {
                        Some(x2) => {
                            match find_tag_name(x2, "mesh") {
                                Some(x3) => {
                                    match find_tag_name(x3, "triangles") {
                                        Some(x4) => {
                                            match find_tag_name(x4, "p") {
                                                Some(x5) => {
                                                    match find_floats(x5) {
                                                        Some(floats) => {
                                                            Some(to_indices(floats))
                                                        },
                                                        None => None
                                                    }
                                                },
                                                None => None
                                            }
                                        },
                                        None => None
                                    }
                                },
                                None => None
                            }
                        },
                        None => None
                    }
                },
                None => None
            }
        },
        _ => None
    }
}

pub fn extract_transform_mat<'a>(source: &'a Collada<'a>) -> Option<[[f32; 4]; 4]> {
    match source {
        Collada::ColladaHeader(b) => {
            match find_tag_name(b, "library_visual_scenes") {
                Some(x1) => {
                    match find_tag_name(x1, "visual_scene") {
                        Some(x2) => {
                            match find_tag_name(x2, "node") {
                                Some(x3) => {
                                    match find_tag_name(x3, "matrix") {
                                        Some(mat) => {
                                            match find_floats(mat){
                                                Some(vec) => {
                                                    to_matrix(vec)
                                                },
                                                None => None
                                            }
                                        },
                                        None => None
                                    }
                                },
                                None => None
                            }
                        },
                        None => None
                    }
                },
                None => None
            }
        },
        _ => None
    }
}

pub fn pack_verts<'a>(pos: Vec<Position>,
                  norm: Vec<Normal>,
                  tex_coords: Vec<TextureCoordinates>,
                  mut indices: Vec<u16>) -> Option<Vec<Vertex>> {
    let mut res : Vec<Vertex> = Vec::new();
    while indices.len() >= 3 {
        let (vertex, rest) = indices.split_at(3);

        let mut vert = Vertex::default();
        
        match pos.get(vertex[0] as usize) {
            Some(position) => { vert.position = position.position; },
            None => { return None; }
        }

        match norm.get(vertex[1] as usize) {
            Some(normal) => { vert.normal = normal.normal; },
            None => { return None; }
        }

        match tex_coords.get(vertex[2] as usize) {
            Some(tex_coords) => { vert.tex_coords = tex_coords.coordinates; },
            None => { return None; }
        }

        indices = Vec::from(rest);
         
        res.push(vert);
    }
    Some(res)
}

pub fn load_model(path: &str, display: &glium::Display) -> (glium::VertexBuffer<Vertex>, [[f32; 4]; 4]){
    let source : String = std::fs::read_to_string(path).unwrap();
    let (_, model): (&str, Collada) = collada_p().parse(&source[..]).unwrap();

    let binding_pos = extract_positions(&model).unwrap();
    let binding_norm = extract_normals(&model).unwrap();
    let binding_tex = extract_texture_coordinates(&model).unwrap();
    let binding_indx = extract_indices(&model).unwrap();

    let vertex_array = pack_verts(binding_pos, binding_norm, binding_tex, binding_indx).unwrap();
    (VertexBuffer::new(display, vertex_array.as_slice()).unwrap(), extract_transform_mat(&model).unwrap())
}

#[test]
fn tag_name_extraction_test() {
    assert_eq!(
        find_tag_name(&Collada::ColladaTag("x", Vec::new(), Box::new(vec![
            Collada::ColladaTag("source", Vec::new(), Box::new(Vec::new())),
            Collada::ColladaTag("source", Vec::new(), Box::new(Vec::new())),
            Collada::ColladaTag("hello", Vec::new(), Box::new(Vec::new())),
        ])), "hello"),
        Some(&Collada::ColladaTag("hello", Vec::new(), Box::new(Vec::new())))
    )
}
