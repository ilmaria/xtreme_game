mod vertex;

use std::error::Error;

pub use self::vertex::Vertex;

pub trait Renderer {
    fn load_vertices(&self, vertices: Vec<Vertex>) -> Result<(), Box<Error>>;
    fn draw_vertices(&self, count: u32, offset: u32) -> Result<(), Box<Error>>;
    fn begin_frame(&mut self) -> Result<(), Box<Error>>;
    fn end_frame(&mut self) -> Result<(), Box<Error>>;
    fn update_resolution(&self, width: u64, height: u64) -> Result<(), Box<Error>>;
    fn change_settings(&self) -> Result<(), Box<Error>>;
}
