use ash::vk;
use ash::extensions::Swapchain;

use std::ptr;
use std::error::Error;

use super::DeviceV10;
use super::InstanceV10;
use super::SurfaceDetails;

pub fn create_swapchain(
    device: &DeviceV10,
    instance: &InstanceV10,
    physical_device: vk::types::PhysicalDevice,
    surface: SurfaceDetails,
) -> Result<(vk::SwapchainKHR, Swapchain), Box<Error>> {
    let desired_image_count = {
        let min_count = surface.capabilities.min_image_count;
        let max_count = surface.capabilities.max_image_count;

        if max_count > 0 && min_count + 1 > max_count {
            max_count
        } else {
            min_count
        }
    };

    let pre_transform = if surface.capabilities.supported_transforms.subset(
        vk::SURFACE_TRANSFORM_IDENTITY_BIT_KHR,
    )
    {
        vk::SURFACE_TRANSFORM_IDENTITY_BIT_KHR
    } else {
        surface.capabilities.current_transform
    };

    let present_mode = surface
        .loader
        .get_physical_device_surface_present_modes_khr(physical_device, surface.khr)?
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
            surface: surface.khr,
            min_image_count: desired_image_count,
            image_color_space: surface.format.color_space,
            image_format: surface.format.format,
            image_extent: surface.resolution.clone(),
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

    Ok((swapchain, swapchain_loader))
}