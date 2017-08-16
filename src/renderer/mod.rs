use ash::vk;

use std::mem;
use std::error::Error;

pub trait Renderer {
    fn load_vertices(&self, vertices: Vec<Vertex>) -> Result<(), Box<Error>>;
    fn draw_vertices(&self, count: u32, offset: u32) -> Result<(), Box<Error>>;
    fn begin_frame(&mut self) -> Result<(), Box<Error>>;
    fn end_frame(&mut self) -> Result<(), Box<Error>>;
    fn update_resolution(&self, width: u64, height: u64) -> Result<(), Box<Error>>;
    fn change_settings(&self) -> Result<(), Box<Error>>;
}

#[derive(Debug, Clone)]
pub struct Model3D {
    pub vertices: Vec<Vertex>,
}

#[derive(Clone, Debug, Copy)]
pub struct Vertex {
    pos: [f32; 4],
    color: [f32; 4],
}

macro_rules! offset_of{
    ($base: path, $field: ident) => {
        {
            unsafe{
                let b: $base = mem::uninitialized();
                (&b.$field as *const _ as isize) - (&b as *const _ as isize)
            }
        }
    }
}

impl Vertex {
    pub fn new(pos: [f32; 4], color: [f32; 4]) -> Vertex {
        Vertex { pos, color }
    }

    pub fn binding_descriptions() -> [vk::VertexInputBindingDescription; 1] {
        [
            vk::VertexInputBindingDescription {
                binding: 0,
                stride: mem::size_of::<Vertex>() as u32,
                input_rate: vk::VertexInputRate::Vertex,
            },
        ]
    }

    pub fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        [
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32g32b32a32Sfloat,
                offset: offset_of!(Vertex, pos) as u32,
            },
            vk::VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: vk::Format::R32g32b32a32Sfloat,
                offset: offset_of!(Vertex, color) as u32,
            },
        ]
    }
}
