use ash::vk;
use ash::version::InstanceV1_0;
use ash::extensions::Surface;

use std::error::Error;

use super::Renderer;
use super::RendererError;

impl Renderer {
    pub fn pick_physical_device(&mut self) -> Result<&mut Renderer, Box<Error>> {
        let instance = self.instance.ok_or(RendererError::NoInstance)?;
        let surface = self.surface.ok_or(RendererError::NoSurface)?;
        let surface_loader = self.surface_loader.ok_or(RendererError::NoSurfaceLoader)?;

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

        self.physical_device = Some(device);
        self.queue_family_index = Some(queue_index);

        Ok(self)
    }
}
