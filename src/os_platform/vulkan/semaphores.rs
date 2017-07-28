use ash::vk;
use ash::version::DeviceV1_0;

use std::ptr;
use std::error::Error;

use super::DeviceV10;

pub fn create_semaphores(device: &DeviceV10) -> Result<(vk::Semaphore, vk::Semaphore), Box<Error>> {
    let semaphore_create_info = vk::SemaphoreCreateInfo {
        s_type: vk::StructureType::SemaphoreCreateInfo,
        p_next: ptr::null(),
        flags: Default::default(),
    };

    let image_available_semaphore = device
        .create_semaphore(&semaphore_create_info, None)?;
    let rendering_complete_semaphore = device
        .create_semaphore(&semaphore_create_info, None)?;

    Ok((image_available_semaphore, rendering_complete_semaphore))
}