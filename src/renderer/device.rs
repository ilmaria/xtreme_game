use ash::vk;
use ash::Instance;
use ash::Device;
use ash::version::{DeviceV1_0, InstanceV1_0, V1_0};
use ash::extensions::Swapchain;

use std::ptr;
use std::error::Error;

use super::VK_INSTANCE;

pub fn new(
    queue_family_index: u32,
    physical_device: vk::PhysicalDevice,
) -> Result<Device<V1_0>, Box<Error>> {
    let features = vk::PhysicalDeviceFeatures {
        shader_clip_distance: 1,
        ..Default::defaul()
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

    let device = unsafe { VK_INSTANCE.create_device(physical_device, &device_create_info, None)? };

    Ok(device)
}
