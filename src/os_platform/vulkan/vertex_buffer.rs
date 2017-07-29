use ash::vk;
use ash::version::{DeviceV1_0, InstanceV1_0};
use ash::util;

use std::ptr;
use std::mem;
use std::error::Error;

use super::DeviceV10;
use super::InstanceV10;
use super::Vertex;
use super::find_memorytype_index;

pub fn create_vertex_buffer(
    device: &DeviceV10,
    instance: &InstanceV10,
    physical_device: vk::PhysicalDevice,
    vertices: Vec<Vertex>,
) -> Result<(), Box<Error>> {
    let buffer_size = (vertices.len() * mem::size_of::<Vertex>()) as u64;
    let vertex_input_buffer = create_buffer(&device, buffer_size)?;
    let memory_requirements = device.get_buffer_memory_requirements(vertex_input_buffer);
    let memory_properties = instance.get_physical_device_memory_properties(physical_device);
    let memory_type = find_memorytype_index(
        &memory_requirements,
        &memory_properties,
        vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk::MEMORY_PROPERTY_HOST_COHERENT_BIT,
    )?;
    let mem_allocate_info = vk::MemoryAllocateInfo {
        s_type: vk::StructureType::MemoryAllocateInfo,
        p_next: ptr::null(),
        allocation_size: memory_requirements.size,
        memory_type_index: memory_type,
    };
    let buffer_memory = device.allocate_memory(&mem_allocate_info, None)?;

    device.bind_buffer_memory(
        vertex_input_buffer,
        buffer_memory,
        0,
    )?;

    let vert_ptr = device.map_memory(
        buffer_memory,
        0,
        memory_requirements.size,
        vk::MemoryMapFlags::empty(),
    )?;
    let mut vert_align = util::Align::new(
        vert_ptr,
        mem::align_of::<Vertex>() as u64,
        memory_requirements.size,
    );
    vert_align.copy_from_slice(&vertices);
    device.unmap_memory(buffer_memory);

    Ok(())
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