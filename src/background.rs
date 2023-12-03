use glium::Display;
use glium::glutin::surface::WindowSurface;

pub struct Background {
    pub texture: glium::texture::SrgbTexture2d,
}

impl Background {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let image = image::load(std::io::Cursor::new(&include_bytes!("../assets/sprites/background-night.png")), image::ImageFormat::Png).unwrap().to_rgb8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgb_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();

        Self {
            texture
        }
    }
}
