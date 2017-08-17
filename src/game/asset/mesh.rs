use cgmath::Vector3;

use std::path::Path;
use std::error::Error;

use renderer::Vertex;
use super::LoadingState;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub path: &'static Path,
    pub loading_state: LoadingState,
    pub descriptors_changed: bool,
}

impl Mesh {
    pub fn new(path: &'static Path) -> Mesh {
        Mesh {
            vertices: vec![],
            indices: vec![],
            path,
            loading_state: LoadingState::Unloaded,
            descriptors_changed: false,
        }
    }

    pub fn load(&mut self) -> Result<(), Box<Error>> {
        self.loading_state = LoadingState::Loaded;
        self.vertices = vec![
            Vertex::new([0.0, 0.0, 0.5, 1.0], [0.2, 0.2, 0.0, 1.0]),
            Vertex::new([0.0, 0.5, 0.0, 1.0], [0.0, 0.6, 0.0, 1.0]),
            Vertex::new([0.5, 0.0, 0.0, 1.0], [0.2, 0.8, 0.0, 1.0]),
            Vertex::new([0.0, 0.0, 0.0, 1.0], [0.2, 0.9, 0.0, 1.0]),

            Vertex::new([0.5, 0.5, 0.5, 1.0], [1.0, 0.0, 0.0, 1.0]),
            Vertex::new([0.5, 0.5, 0.0, 1.0], [0.2, 0.2, 1.0, 1.0]),
            Vertex::new([0.5, 0.0, 0.5, 1.0], [0.7, 0.2, 1.0, 1.0]),
            Vertex::new([0.0, 0.5, 0.5, 1.0], [0.2, 0.2, 0.8, 1.0]),
        ];

        Ok(())
    }

    pub fn unload(&mut self) {}
}
