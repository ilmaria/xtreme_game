use cgmath::Vector3;

use std::path::Path;
use std::error::Error;

use renderer::Vertex;
use super::super::asset;

#[derive(PartialEq, Debug, Clone)]
pub enum LoadingState {
    Unloaded,
    Loaded,
}

#[derive(Debug, Clone)]
pub struct Graphics {
    pub mesh: asset::Mesh,
}

impl Graphics {
    pub fn new() -> Graphics {
        Graphics {
            mesh: asset::Mesh::new(Path::new("/")),
        }
    }
}
