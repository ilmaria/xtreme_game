use ash::vk;
use ash::version::InstanceV1_0;
use ash::extensions::Surface;

use std::error::Error;

use super::InstanceV10;

pub fn pick_physical_device(
    instance: &InstanceV10,
    surface: &vk::SurfaceKHR,
    surface_loader: &Surface,
) -> Result<(vk::PhysicalDevice, u32), Box<Error>> {
    let device = instance
        .enumerate_physical_devices()?
        .iter()
        .map(|device| {
            instance
                .get_physical_device_queue_family_properties(*device)
                .iter()
                .enumerate()
                .filter_map(|(index, ref info)| {
                    let supports_graphic_and_surface =
                        info.queue_flags.subset(vk::QUEUE_GRAPHICS_BIT) &&
                            surface_loader.get_physical_device_surface_support_khr(
                                *device,
                                index as u32,
                                surface.clone(),
                            );
                    match supports_graphic_and_surface {
                        true => Some((*device, index as u32)),
                        _ => None,
                    }
                })
                .nth(0)
        })
        .filter_map(|v| v)
        .nth(0)
        .ok_or("Couldn't find suitable device.")?;

    Ok(device)
}