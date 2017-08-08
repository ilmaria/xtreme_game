mod vertex;

use std::error::Error;

pub use self::vertex::Vertex;

pub trait Renderer {
    fn draw_vertices(&self, vertices: Vec<Vertex>);
    fn display_frame(&mut self) -> Result<(), Box<Error>>;
    fn update_resolution(&self, width: u64, height: u64);
    fn change_settings(&self);
}