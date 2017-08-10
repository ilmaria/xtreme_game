use std::path::Path;
// use std::fs::File;
// use std::io::prelude::*;
use std::error::Error;

use renderer::{Renderer, Vertex};

#[derive(PartialEq, Debug)]
pub enum ModelState {
    Unloaded,
    Loaded,
}

#[derive(Debug)]
pub struct Model3D {
    pub state: ModelState,
    pub path: &'static Path,
    pub vertices: Vec<Vertex>,
    pub offset: u32,
}

impl Model3D {
    pub fn new(path: &'static Path) -> Model3D {
        Model3D {
            state: ModelState::Unloaded,
            path,
            vertices: vec![],
            offset: 0,
        }
    }

    pub fn load(&mut self, renderer: &Renderer) -> Result<(), Box<Error>> {
        if self.state == ModelState::Unloaded {
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

            renderer.load_vertices(vertices)
        } else {
            Ok(())
        }
    }

    pub fn unload(&mut self) {}
}
