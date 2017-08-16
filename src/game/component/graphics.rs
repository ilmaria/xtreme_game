use cgmath::Vector3;

use std::path::Path;

#[derive(PartialEq, Debug, Clone)]
pub enum ModelState {
    Unloaded,
    Loaded,
}

#[derive(Debug, Clone)]
pub struct Graphics {
    pub model_path: &'static Path,
    pub model_state: ModelState,
}

impl Graphics {
    pub fn new(model_path: &'static Path) -> Graphics {
        Graphics {
            model_path,
            model_state: ModelState::Unloaded,
        }
    }

    pub fn load_model(&mut self, path: &'static Path) -> Result<(), Box<Error>> {
        self.state = ModelState::Loaded;
        let vertices = vec![
            Vertex::new([0.0, 0.0, 0.5, 1.0], [0.2, 0.2, 0.0, 1.0]),
            Vertex::new([0.0, 0.5, 0.0, 1.0], [0.0, 0.6, 0.0, 1.0]),
            Vertex::new([0.5, 0.0, 0.0, 1.0], [0.2, 0.8, 0.0, 1.0]),
            Vertex::new([0.0, 0.0, 0.0, 1.0], [0.2, 0.9, 0.0, 1.0]),

            Vertex::new([0.5, 0.5, 0.5, 1.0], [1.0, 0.0, 0.0, 1.0]),
            Vertex::new([0.5, 0.5, 0.0, 1.0], [0.2, 0.2, 1.0, 1.0]),
            Vertex::new([0.5, 0.0, 0.5, 1.0], [0.7, 0.2, 1.0, 1.0]),
            Vertex::new([0.0, 0.5, 0.5, 1.0], [0.2, 0.2, 0.8, 1.0]),
        ];

        self.vertex_count = vertices.len() as u32;

        renderer.load_vertices(vertices)?;
        renderer.draw_vertices(self.vertex_count, self.offset)

        Ok(())
    }

    pub fn unload(&mut self) {}
}