use ash::vk;
use ash::version::DeviceV1_0;

use std::ptr;
use std::error::Error;

pub fn new(device: &DeviceV1_0) -> Result<vk::Semaphore, Box<Error>> {
    let semaphore_create_info = vk::SemaphoreCreateInfo {
        s_type: vk::StructureType::SemaphoreCreateInfo,
        p_next: ptr::null(),
        flags: Default::default(),
    };

    let semaphore = unsafe { device.create_semaphore(&semaphore_create_info, None)? };

    Ok(semaphore)
}