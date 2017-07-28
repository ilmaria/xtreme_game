use ash::vk;
use ash::version::DeviceV1_0;

use std::ptr;
use std::mem;

use super::DeviceV10;
use super::Vertex;

pub fn create_vertex_buffer(device: &DeviceV10, vertices: Vec<Vertex>) {
    let buffer_size = (vertices.len() * mem::size_of::<Vertex>()) as u64;
    let vertex_input_buffer = create_buffer(&device, buffer_size);
}

fn create_buffer(device: &DeviceV10, buffer_size: u64) -> Result<vk::Buffer, vk::Result> {
    let vertex_input_buffer_info = vk::BufferCreateInfo {
        s_type: vk::StructureType::BufferCreateInfo,
        p_next: ptr::null(),
        flags: vk::BufferCreateFlags::empty(),
        size: buffer_size,
        usage: vk::BUFFER_USAGE_VERTEX_BUFFER_BIT,
        sharing_mode: vk::SharingMode::Exclusive,
        queue_family_index_count: 0,
        p_queue_family_indices: ptr::null(),
    };

    device.create_buffer(&vertex_input_buffer_info, None)
}