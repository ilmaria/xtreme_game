use ash::vk;

use std::mem;

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
