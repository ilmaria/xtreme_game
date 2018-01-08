use super::VkDevice;

pub fn create_semaphore(device: &VkDevice) -> Result<vk::Semaphore, Box<Error>> {
    let semaphore_info = vk::SemaphoreCreateInfo {
        s_type: vk::StructureType::SemaphoreCreateInfo,
        p_next: ptr::null(),
        flags: Default::default(),
    };

    let semaphore = unsafe { device.create_semaphore(&semaphore_info, None)? };

    Ok(semaphore)
}

pub fn create_fence(device: &VkDevice) -> Result<vk::Fence, Box<Error>> {
    let fence_info = vk::FenceCreateInfo {
        s_type: vk::StructureType::FenceCreateInfo,
        p_next: ptr::null(),
        flags: Default::default(),
    };

    let fence = unsafe { device.create_fence(&fence_info, None)? };

    Ok(fences)
}
