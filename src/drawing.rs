use std::cmp::Ordering;

use glium::framebuffer::{SimpleFrameBuffer, DepthRenderBuffer};
use glium::VertexBuffer;
use glium::index::{PrimitiveType, NoIndices};
use glium::texture::DepthFormat;
use glium::texture::texture2d::Texture2d;
use glium::implement_vertex;
use glium::uniform;
use glium::Surface;

use crate::event_handling::camera_transformations::Camera;
use shader_compilation::ShaderProg;
use texture::Texture;

pub mod mesh;
pub mod shader_compilation;
pub mod texture;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
struct SpriteVertex {
    position: [f32; 2]
}

implement_vertex!(SpriteVertex, position);

pub fn render_meshes(meshes: Vec<&mesh::Mesh>,
                     camera: &Camera,
                     display: &glium::Display,
                     shaders: Vec<&ShaderProg>,
                     postpr_shader: Option<&ShaderProg>,
                     textures: Vec<&Texture>) {
    let mut target = display.draw();
    let target_dimensions = target.get_dimensions();
    let target_color = Texture2d::empty(display,
                                        target_dimensions.0,
                                        target_dimensions.1).unwrap();
    let target_depth = DepthRenderBuffer::new(display,
                                              DepthFormat::I24,
                                              target_dimensions.0,
                                              target_dimensions.1).unwrap();
    let mut framebuffer = SimpleFrameBuffer::with_depth_buffer(display,
                                                               &target_color,
                                                               &target_depth).unwrap();
    
    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
    framebuffer.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
    
    let params = glium::DrawParameters {
        depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
        .. Default::default()
    };
    
    for mesh in meshes.iter() {
        let shader_id = mesh.shader_id;
        let current_shader_index = shaders.binary_search_by(|prog| {
            if prog.id < shader_id {
                Ordering::Less
            } else if prog.id > shader_id {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }).unwrap();

        let current_shader = shaders.get(current_shader_index).unwrap();

        let current_texture_index = textures.binary_search_by( |tex| {
            let tex_id = mesh.texture_id;
            if tex.get_id() < tex_id {
                Ordering::Less
            } else if tex.get_id() > tex_id {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }).unwrap();

        let current_texture = textures.get(current_texture_index).unwrap();
        
        match &mesh.vertex_buf {
            Some(buf) => { framebuffer.draw(buf,
                                            NoIndices(PrimitiveType::TrianglesList),
                                            &current_shader.prog,
                                            &uniform! {
                                                camera_pos: camera.position,
                                                camera_right: camera.right,
                                                camera_up: camera.up,
                                                camera_front: camera.front,
                                                camera_fov: camera.fov,
                                                aspect_ratio: camera.view_aspect_ratio,
                                                trans_mat: mesh.transform_mat,
                                                offset: mesh.offset,
                                                scale: mesh.scale,
                                                tex: current_texture.get_texture(),
                                            },
                                            &params).unwrap();
            },
            None => {}
        }
    }

    let postpr_vertex_buffer = VertexBuffer::new(display,
                                                 &[
                                                     SpriteVertex{ position: [-1.0, -1.0] },
                                                     SpriteVertex{ position: [-1.0,  1.0] },
                                                     SpriteVertex{ position: [ 1.0,  1.0] },
                                                     SpriteVertex{ position: [ 1.0, -1.0] },
                                                     SpriteVertex{ position: [-1.0, -1.0] },
                                                     SpriteVertex{ position: [ 1.0,  1.0] }
                                                 ]).unwrap();
   
    match postpr_shader {
        Some(prog) => {
            let uniforms = uniform! {
                tex: target_color
            };
            target.draw(&postpr_vertex_buffer,
                        NoIndices(PrimitiveType::TrianglesList),
                        prog.get_prog(),
                        &uniforms,
                        &Default::default()).unwrap();
        },
        None => {}
    }
    
    target.finish().unwrap();
}
