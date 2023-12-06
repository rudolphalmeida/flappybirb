use glium::Display;
use glium::glutin::surface::WindowSurface;

pub struct Texture {
    texture: glium::texture::SrgbTexture2d,
    pub size: (u32, u32),
}

impl Texture {
    pub fn from_bytes(bytes: &[u8], display: &Display<WindowSurface>) -> Self {
        let image = image::load(std::io::Cursor::new(bytes), image::ImageFormat::Png).unwrap().to_rgba8();
        let size = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), size);
        let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();

        Self {
            texture,
            size
        }
    }

    pub fn texture_id(&self) -> &glium::texture::SrgbTexture2d {
        &self.texture
    }
}
