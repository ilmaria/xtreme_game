use ash::vk;
use ash::version::{DeviceV1_0, InstanceV1_0};
use ash::util;

use std::ptr;
use std::mem;
use std::error::Error;

use super::VulkanRenderer;
use super::RendererError;
use super::Vertex;
use super::find_memorytype_index;

impl VulkanRenderer {
    pub fn create_vertex_buffer(&self, vertices: Vec<Vertex>) -> Result<vk::Buffer, Box<Error>> {
        let device = self.device.as_ref().ok_or(RendererError::NoDevice)?;
        let instance = self.instance.as_ref().ok_or(RendererError::NoInstance)?;
        let physical_device = self.physical_device.ok_or(RendererError::NoPhysicalDevice)?;

        let buffer_size = (vertices.len() * mem::size_of::<Vertex>()) as u64;

        let (staging_buffer, staging_buffer_memory) = self.create_buffer(
            buffer_size,
            vk::BUFFER_USAGE_TRANSFER_SRC_BIT,
            vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT |
                vk::MEMORY_PROPERTY_HOST_COHERENT_BIT,
        )?;


        unsafe {
            let vert_ptr = device.map_memory(
                staging_buffer_memory,
                0,
                buffer_size,
                vk::MemoryMapFlags::empty(),
            )?;
            let mut vert_align =
                util::Align::new(vert_ptr, mem::align_of::<Vertex>() as u64, buffer_size);
            vert_align.copy_from_slice(&vertices);
            device.unmap_memory(staging_buffer_memory);
        }

        let (vertex_buffer, vertex_buffer_memory) = self.create_buffer(
            buffer_size,
            vk::BUFFER_USAGE_TRANSFER_DST_BIT |
                vk::BUFFER_USAGE_VERTEX_BUFFER_BIT,
            vk::MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
        )?;

        self.copy_buffer(staging_buffer, vertex_buffer, buffer_size)?;

        unsafe {
            device.destroy_buffer(staging_buffer, None);
            device.free_memory(staging_buffer_memory, None);
        }

        Ok(vertex_buffer)
    }

    fn create_buffer(
        &self,
        buffer_size: u64,
        usage: vk::BufferUsageFlags,
        properties: vk::MemoryPropertyFlags,
    ) -> Result<(vk::Buffer, vk::DeviceMemory), Box<Error>> {
        let device = self.device.as_ref().ok_or(RendererError::NoDevice)?;
        let instance = self.instance.as_ref().ok_or(RendererError::NoInstance)?;
        let physical_device = self.physical_device.ok_or(RendererError::NoPhysicalDevice)?;

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
        let memory_properties = instance.get_physical_device_memory_properties(physical_device);
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

        Ok((buffer, buffer_memory))
    }

    fn copy_buffer(&self, src: vk::Buffer, dst: vk::Buffer, size: u64) -> Result<(), Box<Error>> {
        let device = self.device.as_ref().ok_or(RendererError::NoDevice)?;
        let command_pool = self.command_pool.ok_or(RendererError::NoCommandPool)?;
        let present_queue = self.present_queue.ok_or(RendererError::NoPresentQueue)?;

        let alloc_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::CommandBufferAllocateInfo,
            p_next: ptr::null(),
            level: vk::CommandBufferLevel::Primary,
            command_pool: command_pool,
            command_buffer_count: 1,
        };

        unsafe {
            let command_buffer = device.allocate_command_buffers(&alloc_info)?[0];

            self.submit_commands(
                command_buffer,
                present_queue,
                &[],
                &[],
                &[],
                |&device, command_buffer| {
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
}
