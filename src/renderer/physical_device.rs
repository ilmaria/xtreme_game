use ash::vk;
use ash::version::InstanceV1_0;

use std::error::Error;

use super::surface::Surface;
use super::VK_INSTANCE;


pub fn new() -> Result<vk::PhysicalDevice, Box<Error>> {
    let device = VK_INSTANCE
        .enumerate_physical_devices()?
        .into_iter()
        .next()
        .ok_or("Couldn't find suitable physical device.")?;

    Ok(PhysicalDevice(device))
}

pub fn graphics_queue_index(
    physical_device: &vk::PhysicalDevice,
    surface: &Surface,
) -> Option<u32> {
    VK_INSTANCE
        .get_physical_device_queue_family_properties(physical_device)
        .iter()
        .enumerate()
        .filter_map(|(index, ref info)| {
            let supports_graphics = info.queue_flags.subset(vk::QUEUE_GRAPHICS_BIT)
                && surface.loader().get_physical_device_surface_support_khr(
                    physical_device,
                    index as u32,
                    surface.handle(),
                );
            match supports_graphics {
                true => Some(index as u32),
                _ => None,
            }
        })
        .nth(0)
}
