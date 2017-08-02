use ash::vk;
use ash::extensions::Swapchain;

use std::ptr;
use std::error::Error;

use super::VulkanRenderer;
use super::RendererError;

impl VulkanRenderer {
pub fn create_swapchain(&mut self) -> Result<&mut VulkanRenderer, Box<Error>> {
    let instance = self.instance.as_ref().ok_or(RendererError::NoInstance)?;
    let surface = self.surface.ok_or(RendererError::NoSurface)?;
    let surface_loader = self.surface_loader.ok_or(RendererError::NoSurfaceLoader)?;
    let surface_format = self.surface_format.ok_or(RendererError::NoSurfaceFormat)?;
    let surface_resolution = self.surface_resolution.ok_or(RendererError::NoSurfaceResolution)?;
    let device = self.device.as_ref().ok_or(RendererError::NoDevice)?;
    let physical_device = self.physical_device.ok_or(RendererError::NoPhysicalDevice)?;

    let surface_capabilities = surface_loader.get_physical_device_surface_capabilities_khr(
        physical_device,
        surface,
    )?;

    let desired_image_count = {
        let min_count = surface_capabilities.min_image_count;
        let max_count = surface_capabilities.max_image_count;

        if max_count > 0 && min_count + 1 > max_count {
            max_count
        } else {
            min_count
        }
    };

    let pre_transform = if surface_capabilities.supported_transforms.subset(
        vk::SURFACE_TRANSFORM_IDENTITY_BIT_KHR,
    )
    {
        vk::SURFACE_TRANSFORM_IDENTITY_BIT_KHR
    } else {
        surface_capabilities.current_transform
    };

    let present_mode = surface_loader
        .get_physical_device_surface_present_modes_khr(physical_device, surface)?
        .iter()
        .cloned()
        .find(|&mode| mode == vk::PresentModeKHR::Mailbox)
        .unwrap_or(vk::PresentModeKHR::Fifo);

    let swapchain_loader = Swapchain::new(instance, device).map_err(
        |_| "Unable to load swapchain",
    )?;

    let swapchain = {
        let swapchain_create_info = vk::SwapchainCreateInfoKHR {
            s_type: vk::StructureType::SwapchainCreateInfoKhr,
            p_next: ptr::null(),
            flags: Default::default(),
            surface: surface,
            min_image_count: desired_image_count,
            image_color_space: surface_format.color_space,
            image_format: surface_format.format,
            image_extent: surface_resolution.clone(),
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

        unsafe {
            swapchain_loader.create_swapchain_khr(
                &swapchain_create_info,
                None,
            )?
        }
    };

    self.swapchain = Some(swapchain);
    self.swapchain_loader = Some(swapchain_loader);

    Ok(self)
}
}