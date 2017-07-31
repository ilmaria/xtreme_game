use ash::vk;
use ash::version::DeviceV1_0;

use std::ptr;
use std::error::Error;

use super::Renderer;
use super::RendererError;

impl Renderer {
    pub fn create_command_pool(&mut self) -> Result<&mut Renderer, Box<Error>> {
        let instance = self.instance.ok_or(RendererError::NoInstance)?;
        let queue_family_index = self.queue_family_index.ok_or(
            RendererError::NoqueueFamilyIndex,
        )?;
        let device = self.device.ok_or(RendererError::NoDevice)?;

        let pool_create_info = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::CommandPoolCreateInfo,
            p_next: ptr::null(),
            flags: vk::COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
            queue_family_index,
        };

        let command_pool = device.create_command_pool(&pool_create_info, None)?;

        self.command_pool = Some(command_pool);

        Ok(self)
    }

    pub fn create_command_buffers(&mut self) -> Result<&mut Renderer, Box<Error>> {
        let command_pool = self.command_pool.ok_or(RendererError::NoCommandPool)?;
        let device = self.device.ok_or(RendererError::NoDevice)?;
        let framebuffers = self.framebuffers.ok_or(RendererError::NoFramebuffers)?;

        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::CommandBufferAllocateInfo,
            p_next: ptr::null(),
            command_buffer_count: framebuffers.len() as u32,
            command_pool,
            level: vk::CommandBufferLevel::Primary,
        };

        let command_buffers = device.allocate_command_buffers(
            &command_buffer_allocate_info,
        )?;

        self.command_buffers = Some(command_buffers);

        Ok(self)
    }
}