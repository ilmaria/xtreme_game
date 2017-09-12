use ash::vk;
use ash::Instance;
use ash::version::{DeviceV1_0, InstanceV1_0, V1_0};
use ash::util;

use std::ptr;
use std::mem;
use std::error::Error;

use super::Vertex;
use super::find_memorytype_index;

const BUFFER_SIZE: u64 = 256 * 1024 * 1024;

pub struct Allocator {
    buffers: Vec<Buffer>,
    physical_device: vk::PhysicalDevice,
}

pub struct Buffer {
    pub buf: vk::Buffer,
    pub memory: vk::DeviceMemory,
    pub size: u64,
}

impl Allocator {
    pub fn new(physical_device: vk::PhysicalDevice) -> Allocator {
        Allocator {
            buffers: vec![],
            physical_device,
        }
    }

    pub fn create_buffer(
        &mut self,
        device: &DeviceV1_0,
        instance: &Instance<V1_0>,
        buffer_size: u64,
        usage: vk::BufferUsageFlags,
        properties: vk::MemoryPropertyFlags,
    ) -> Result<usize, Box<Error>> {
        let buffer_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BufferCreateInfo,
            p_next: ptr::null(),
            flags: vk::BufferCreateFlags::empty(),
            size: buffer_size,
            usage: usage,
            sharing_mode: vk::SharingMode::Exclusive,
            queue_family_index_count: 0,
            p_queue_family_indices: ptr::null(),
        };

        let buffer = unsafe { device.create_buffer(&buffer_info, None)? };

        let memory_requirements = device.get_buffer_memory_requirements(buffer);
        let memory_properties =
            instance.get_physical_device_memory_properties(self.physical_device);
        let memory_type =
            find_memorytype_index(&memory_requirements, &memory_properties, properties)?;
        let mem_allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MemoryAllocateInfo,
            p_next: ptr::null(),
            allocation_size: memory_requirements.size,
            memory_type_index: memory_type,
        };

        let buffer_memory = unsafe {
            let buffer_memory = device.allocate_memory(&mem_allocate_info, None)?;
            device.bind_buffer_memory(buffer, buffer_memory, 0)?;
            buffer_memory
        };

        self.buffers.push(Buffer {
            buf: buffer,
            memory: buffer_memory,
            size: buffer_size,
        });

        Ok(self.buffers.len() - 1)
    }
}

pub fn copy_vertices_to_device(
    device: &DeviceV1_0,
    instance: &Instance<V1_0>,
    physical_device: vk::PhysicalDevice,
    command_pool: vk::CommandPool,
    present_queue: vk::Queue,
    staging_buffer: &Buffer,
    vertex_buffer: &Buffer,
    vertices: Vec<Vertex>,
) -> Result<(), Box<Error>> {
    unsafe {
        let vert_ptr = device.map_memory(
            staging_buffer.memory,
            0,
            staging_buffer.size,
            vk::MemoryMapFlags::empty(),
        )?;
        let mut vert_align = util::Align::new(
            vert_ptr,
            mem::align_of::<Vertex>() as u64,
            staging_buffer.size,
        );
        vert_align.copy_from_slice(&vertices);
        device.unmap_memory(staging_buffer.memory);
    }

    copy_buffer(
        device,
        command_pool,
        present_queue,
        staging_buffer.buf,
        vertex_buffer.buf,
        staging_buffer.size,
    )?;

    Ok(())
}

fn copy_buffer(
    device: &DeviceV1_0,
    command_pool: vk::CommandPool,
    present_queue: vk::Queue,
    src: vk::Buffer,
    dst: vk::Buffer,
    size: u64,
) -> Result<(), Box<Error>> {
    let alloc_info = vk::CommandBufferAllocateInfo {
        s_type: vk::StructureType::CommandBufferAllocateInfo,
        p_next: ptr::null(),
        level: vk::CommandBufferLevel::Primary,
        command_pool: command_pool,
        command_buffer_count: 1,
    };

    unsafe {
        let command_buffer = device.allocate_command_buffers(&alloc_info)?[0];

        super::command::submit(
            device,
            command_buffer,
            present_queue,
            &[],
            &[],
            &[],
            |device, command_buffer| {
                let copy_regions = [
                    vk::BufferCopy {
                        src_offset: 0,
                        dst_offset: 0,
                        size,
                    },
                ];
                device.cmd_copy_buffer(command_buffer, src, dst, &copy_regions);
            },
        )?;

        device.free_command_buffers(command_pool, &[command_buffer]);
    }

    Ok(())
}
