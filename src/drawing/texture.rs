use glium::texture::RawImage2d;
use glium::texture::texture2d::Texture2d;
use glium::Display;

use std::io::BufReader;
use std::fs::File;
use std::clone::Clone;

pub struct Texture {
    id: u16,
    texture: Texture2d
}

impl Texture {
    pub fn get_id(&self) -> u16 {
        self.id
    }

    pub fn get_texture(&self) -> &Texture2d {
        &self.texture
    }
    
    pub fn from_file(id: u16, display: &Display) -> Texture {
        let mut pathstr = std::string::String::new();
        pathstr.push_str("./resources/textures/tex");
        pathstr.push_str(id.to_string().as_str());
        pathstr.push_str(".png");
        let path = std::path::Path::new(&pathstr);

        let image = image::load(BufReader::new(File::open(path).unwrap()),
                                image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        Texture {
            id: id,
            texture: Texture2d::new(display, image).unwrap()
        }
    }
}
