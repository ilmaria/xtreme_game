use ash::vk;
use ash::version::DeviceV1_0;

use std::ptr;
use std::error::Error;

use super::DeviceV10;

pub fn create_command_pool(
    device: &DeviceV10,
    queue_family_index: u32,
) -> Result<vk::CommandPool, Box<Error>> {
    let pool_create_info = vk::CommandPoolCreateInfo {
        s_type: vk::StructureType::CommandPoolCreateInfo,
        p_next: ptr::null(),
        flags: vk::COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
        queue_family_index,
    };

    let command_pool = device.create_command_pool(&pool_create_info, None)?;

    Ok(command_pool)
}

pub fn create_command_buffers(
    device: &DeviceV10,
    command_pool: vk::CommandPool,
    buffer_count: u32,
) -> Result<Vec<vk::CommandBuffer>, Box<Error>> {
    let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
        s_type: vk::StructureType::CommandBufferAllocateInfo,
        p_next: ptr::null(),
        command_buffer_count: buffer_count,
        command_pool,
        level: vk::CommandBufferLevel::Primary,
    };

    let command_buffers = device.allocate_command_buffers(&command_buffer_allocate_info)?;

    Ok(command_buffers)
}