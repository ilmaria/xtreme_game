use ash::vk;
use ash::Instance;
use ash::Device;
use ash::version::{InstanceV1_0, DeviceV1_0, V1_0};
use ash::extensions::Swapchain;

use std::ptr;
use std::error::Error;

use super::VulkanRenderer;
use super::RendererError;

impl VulkanRenderer {
    pub fn create_logical_device(
        instance: &Instance<V1_0>,
        queue_family_index: u32,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Device<V1_0>, Box<Error>> {

        let features = vk::PhysicalDeviceFeatures {
            shader_clip_distance: 1,
            ..Default::default()
        };

        let queue_info = {
            let priorities = [1.0];

            vk::DeviceQueueCreateInfo {
                s_type: vk::StructureType::DeviceQueueCreateInfo,
                p_next: ptr::null(),
                flags: Default::default(),
                queue_family_index: queue_family_index,
                p_queue_priorities: priorities.as_ptr(),
                queue_count: priorities.len() as u32,
            }
        };

        let device_extension_names = [Swapchain::name().as_ptr()];

        let device_create_info = vk::DeviceCreateInfo {
            s_type: vk::StructureType::DeviceCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
            queue_create_info_count: 1,
            p_queue_create_infos: &queue_info,
            enabled_layer_count: 0,
            pp_enabled_layer_names: ptr::null(),
            enabled_extension_count: device_extension_names.len() as u32,
            pp_enabled_extension_names: device_extension_names.as_ptr(),
            p_enabled_features: &features,
        };

        let device = unsafe {
            instance.create_device(
                physical_device,
                &device_create_info,
                None,
            )?
        };

        Ok(device)
    }

    pub fn create_queue(device: &DeviceV1_0, queue_family_index: u32) -> vk::Queue {
        unsafe { device.get_device_queue(queue_family_index, 0) }
    }
}