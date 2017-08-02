use ash::vk;
use ash::Device;
use ash::Instance;
use ash::version::{DeviceV1_0, V1_0};

use std::ptr;
use std::error::Error;
use std::u64;

use super::VulkanRenderer;
use super::RendererError;

impl VulkanRenderer {
    pub fn create_command_pool(
        device: &DeviceV1_0,
        instance: &Instance<V1_0>,
        queue_family_index: u32,
    ) -> Result<vk::CommandPool, Box<Error>> {
        let pool_create_info = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::CommandPoolCreateInfo,
            p_next: ptr::null(),
            flags: vk::COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
            queue_family_index,
        };

        let command_pool = unsafe { device.create_command_pool(&pool_create_info, None)? };

        Ok(command_pool)
    }

    pub fn create_command_buffers(
        device: &DeviceV1_0,
        command_pool: vk::CommandPool,
        framebuffers: Vec<vk::Framebuffer>,
    ) -> Result<Vec<vk::CommandBuffer>, Box<Error>> {

        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::CommandBufferAllocateInfo,
            p_next: ptr::null(),
            command_buffer_count: framebuffers.len() as u32,
            command_pool,
            level: vk::CommandBufferLevel::Primary,
        };

        let command_buffers = unsafe {
            device.allocate_command_buffers(
                &command_buffer_allocate_info,
            )?
        };

        Ok(command_buffers)
    }

    pub fn submit_commands<F: FnOnce(&Device<V1_0>, vk::CommandBuffer)>(
        &self,
        command_buffer: vk::CommandBuffer,
        submit_queue: vk::Queue,
        wait_mask: &[vk::PipelineStageFlags],
        wait_semaphores: &[vk::Semaphore],
        signal_semaphores: &[vk::Semaphore],
        f: F,
    ) -> Result<(), Box<Error>> {
        let device = self.device.as_ref().ok_or(RendererError::NoDevice)?;

        unsafe {
            device.reset_command_buffer(
                command_buffer,
                vk::COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT,
            )?;
        }
        let command_buffer_begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::CommandBufferBeginInfo,
            p_next: ptr::null(),
            p_inheritance_info: ptr::null(),
            flags: vk::COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
        };

        unsafe {
            device.begin_command_buffer(
                command_buffer,
                &command_buffer_begin_info,
            )?;
        }

        f(&device, command_buffer);

        unsafe {
            device.end_command_buffer(command_buffer)?;
        }

        let fence_create_info = vk::FenceCreateInfo {
            s_type: vk::StructureType::FenceCreateInfo,
            p_next: ptr::null(),
            flags: vk::FenceCreateFlags::empty(),
        };
        let submit_fence = unsafe { device.create_fence(&fence_create_info, None)? };
        let submit_info = vk::SubmitInfo {
            s_type: vk::StructureType::SubmitInfo,
            p_next: ptr::null(),
            wait_semaphore_count: wait_semaphores.len() as u32,
            p_wait_semaphores: wait_semaphores.as_ptr(),
            p_wait_dst_stage_mask: wait_mask.as_ptr(),
            command_buffer_count: 1,
            p_command_buffers: &command_buffer,
            signal_semaphore_count: signal_semaphores.len() as u32,
            p_signal_semaphores: signal_semaphores.as_ptr(),
        };
        unsafe {
            device.queue_submit(
                submit_queue,
                &[submit_info],
                submit_fence,
            )?;

            device.wait_for_fences(&[submit_fence], true, u64::MAX)?;
            device.destroy_fence(submit_fence, None);
        }

        Ok(())
    }
}