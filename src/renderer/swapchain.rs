use ash::vk;
use ash::Instance;
use ash::Device;
use ash::version::{DeviceV1_0, V1_0};
use ash::extensions as ext;

use std::ptr;
use std::error::Error;

use super::VK_INSTANCE;
use super::VkDevice;
use super::surface::Surface;

pub struct Swapchain {
    handle: vk::SwapchainKHR,
    loader: ext:Swapchain,
    current_image: u32,
}

impl Swapchain {
    pub fn new(
        device: &VkDevice,
        physical_device: vk::PhysicalDevice,
        surface: Surface,
    ) -> Result<Swapchain, Box<Error>> {
        let loader = ext::Swapchain::new(VK_INSTANCE, device).map_err(|_| "Unable to load swapchain")?;

        let capabilities = surface.capabilities(physical_device)?;

        let desired_image_count = {
            let min_count = capabilities.min_image_count;
            let max_count = capabilities.max_image_count;

            if max_count > 0 && min_count + 1 > max_count {
                max_count
            } else {
                min_count + 1
            }
        };

        let pre_transform = if capabilities
            .supported_transforms
            .subset(vk::SURFACE_TRANSFORM_IDENTITY_BIT_KHR)
        {
            vk::SURFACE_TRANSFORM_IDENTITY_BIT_KHR
        } else {
            capabilities.current_transform
        };

        let present_mode = surface.present_modes(physical_device)?
            .cloned()
            .find(|&mode| mode == vk::PresentModeKHR::Mailbox)
            .unwrap_or(vk::PresentModeKHR::Fifo);

        let format = surface.format();

        let handle = {
            let swapchain_create_info = vk::SwapchainCreateInfoKHR {
                s_type: vk::StructureType::SwapchainCreateInfoKhr,
                p_next: ptr::null(),
                flags: Default::default(),
                surface: surface.handle(),
                min_image_count: desired_image_count,
                image_color_space: format.color_space,
                image_format: format.format,
                image_extent: surface.extent().clone(),
                image_usage: vk::IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
                image_sharing_mode: vk::SharingMode::Exclusive,
                pre_transform: pre_transform,
                composite_alpha: vk::COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
                present_mode: present_mode,
                clipped: 1,
                old_swapchain: vk::SwapchainKHR::null(),
                image_array_layers: 1,
                p_queue_family_indices: ptr::null(),
                queue_family_index_count: 0,
            };

            unsafe { loader.create_swapchain_khr(&swapchain_create_info, None)? }
        };

        Ok(Swapchain {
            handle,
            loader,
            current_image: 0,
        })
    }

    pub fn images(&self) -> Result<Vec<vk::Image>, Box<Error>> {
        self.loader().get_swapchain_images_khr(self.handle)
    }
}
