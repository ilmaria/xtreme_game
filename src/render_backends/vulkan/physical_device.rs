use ash::vk;
use ash::Instance;
use ash::version::{InstanceV1_0, V1_0};
use ash::extensions::Surface;

use std::error::Error;

pub fn new_with_queue(
    instance: &Instance<V1_0>,
    surface: vk::SurfaceKHR,
    surface_loader: &Surface,
) -> Result<(vk::PhysicalDevice, u32), Box<Error>> {
    let (device, queue_index) = instance
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
                                surface,
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

    Ok((device, queue_index))
}
