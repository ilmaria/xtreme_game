use ash::vk;
use ash::Device;
use ash::Instance;
use ash::version::{DeviceV1_0, V1_0};

use std::ptr;
use std::error::Error;
use std::u64;

pub fn new(
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