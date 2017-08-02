use ash::vk;
use ash::version::DeviceV1_0;

use std::ptr;
use std::error::Error;

use super::VulkanRenderer;
use super::RendererError;

impl VulkanRenderer {
    pub fn create_semaphores(&mut self) -> Result<&mut VulkanRenderer, Box<Error>> {
        let device = self.device.ok_or(RendererError::NoDevice)?;

        let semaphore_create_info = vk::SemaphoreCreateInfo {
            s_type: vk::StructureType::SemaphoreCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
        };

        unsafe {
            let image_available_semaphore = device.create_semaphore(&semaphore_create_info, None)?;
            let rendering_complete_semaphore =
                device.create_semaphore(&semaphore_create_info, None)?;

            self.image_available_semaphore = Some(image_available_semaphore);
            self.rendering_complete_semaphore = Some(rendering_complete_semaphore);
        }

        Ok(self)
    }
}