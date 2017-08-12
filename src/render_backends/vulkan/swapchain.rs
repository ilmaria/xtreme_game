use ash::vk;
use ash::Instance;
use ash::Device;
use ash::version::{DeviceV1_0, V1_0};
use ash::extensions::{Surface, Swapchain};

use std::ptr;
use std::error::Error;

pub fn new_loader(
    device: &Device<V1_0>,
    instance: &Instance<V1_0>,
) -> Result<Swapchain, Box<Error>> {
    let swapchain_loader = Swapchain::new(instance, device)
        .map_err(|_| "Unable to load swapchain")?;

    Ok(swapchain_loader)
}

pub fn new(
    swapchain_loader: &Swapchain,
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
    surface_loader: &Surface,
    surface_format: &vk::SurfaceFormatKHR,
    surface_resolution: &vk::Extent2D,
) -> Result<vk::SwapchainKHR, Box<Error>> {
    let surface_capabilities = surface_loader
        .get_physical_device_surface_capabilities_khr(physical_device, surface)?;

    let desired_image_count = {
        let min_count = surface_capabilities.min_image_count;
        let max_count = surface_capabilities.max_image_count;

        if max_count > 0 && min_count + 1 > max_count {
            max_count
        } else {
            min_count
        }
    };

    let pre_transform = if surface_capabilities
        .supported_transforms
        .subset(vk::SURFACE_TRANSFORM_IDENTITY_BIT_KHR)
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
            swapchain_loader
                .create_swapchain_khr(&swapchain_create_info, None)?
        }
    };

    Ok(swapchain)
}

pub fn new_semaphores(
    device: &DeviceV1_0,
    swapchain_len: usize,
) -> Result<Vec<vk::Semaphore>, Box<Error>> {
    let mut semaphores = vec![];

    for _ in 0..swapchain_len {
        let semaphore_info = vk::SemaphoreCreateInfo {
            s_type: vk::StructureType::SemaphoreCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
        };

        let semaphore = unsafe { device.create_semaphore(&semaphore_info, None)? };
        semaphores.push(semaphore);
    }

    Ok(semaphores)
}

pub fn new_fences(device: &DeviceV1_0, swapchain_len: usize) -> Result<Vec<vk::Fence>, Box<Error>> {
    let mut fences = vec![];

    for _ in 0..swapchain_len {
        let fence_info = vk::FenceCreateInfo {
            s_type: vk::StructureType::FenceCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
        };

        let fence = unsafe { device.create_fence(&fence_info, None)? };
        fences.push(fence);
    }

    Ok(fences)
}
