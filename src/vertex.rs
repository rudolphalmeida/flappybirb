use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    uv: [f32; 2],
}

impl Vertex {
    fn new(position: [f32; 2], uv: [f32; 2]) -> Self {
        Self { position, uv }
    }

    pub fn sprite_rectangle() -> [Vertex; 6] {
        [
            Vertex::new([0.0, 1.0], [0.0, 1.0]),
            Vertex::new([1.0, 0.0], [1.0, 0.0]),
            Vertex::new([0.0, 0.0], [0.0, 0.0]),
            Vertex::new([0.0, 1.0], [0.0, 1.0]),
            Vertex::new([1.0, 1.0], [1.0, 1.0]),
            Vertex::new([1.0, 0.0], [1.0, 0.0]),
        ]
    }
}

implement_vertex!(Vertex, position, uv);
