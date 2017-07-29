use ash::vk;
use ash::version::{DeviceV1_0, InstanceV1_0};
use ash::extensions::Swapchain;

use std::ptr;
use std::error::Error;

use super::Renderer;
use super::RendererError;
use super::find_memorytype_index;

impl Renderer {
    pub fn create_image_views(&mut self) -> Result<&mut Renderer, Box<Error>> {
        let surface_format = self.surface_format.ok_or(RendererError::NoSurfaceFormat)?;
        let device = self.device.ok_or(RendererError::NoDevice)?;
        let swapchain = self.swapchain.ok_or(RendererError::NoSwapchain)?;
        let swapchain_loader = self.swapchain_loader.ok_or(
            RendererError::NoSwapchainLoader,
        )?;

        let image_views = swapchain_loader
            .get_swapchain_images_khr(swapchain)?
            .iter()
            .map(|&image| {
                let create_view_info = vk::ImageViewCreateInfo {
                    s_type: vk::StructureType::ImageViewCreateInfo,
                    p_next: ptr::null(),
                    flags: Default::default(),
                    view_type: vk::ImageViewType::Type2d,
                    format: surface_format.format,
                    components: vk::ComponentMapping {
                        r: vk::ComponentSwizzle::Identity,
                        g: vk::ComponentSwizzle::Identity,
                        b: vk::ComponentSwizzle::Identity,
                        a: vk::ComponentSwizzle::Identity,
                    },
                    subresource_range: vk::ImageSubresourceRange {
                        aspect_mask: vk::IMAGE_ASPECT_COLOR_BIT,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                    image: image,
                };
                device.create_image_view(&create_view_info, None)
            })
            .collect()?;

        self.present_image_views = Some(image_views);

        Ok(self)
    }

    pub fn create_depth_view(&mut self) -> Result<&mut Renderer, Box<Error>> {
        let surface_resolution = self.surface_resolution.ok_or(
            RendererError::NoSurfaceResolution,
        )?;
        let instance = self.instance.ok_or(RendererError::NoInstance)?;
        let device = self.device.ok_or(RendererError::NoDevice)?;
        let physical_device = self.physical_device.ok_or(RendererError::NoPhysicalDevice)?;

        let depth_image_create_info = vk::ImageCreateInfo {
            s_type: vk::StructureType::ImageCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
            image_type: vk::ImageType::Type2d,
            format: vk::Format::D16Unorm,
            extent: vk::Extent3D {
                width: surface_resolution.width,
                height: surface_resolution.height,
                depth: 1,
            },
            mip_levels: 1,
            array_layers: 1,
            samples: vk::SAMPLE_COUNT_1_BIT,
            tiling: vk::ImageTiling::Optimal,
            usage: vk::IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT,
            sharing_mode: vk::SharingMode::Exclusive,
            queue_family_index_count: 0,
            p_queue_family_indices: ptr::null(),
            initial_layout: vk::ImageLayout::Undefined,
        };
        let depth_image = device.create_image(&depth_image_create_info, None)?;

        let device_memory_properties =
            instance.get_physical_device_memory_properties(physical_device);
        let depth_image_memory_req = device.get_image_memory_requirements(depth_image);
        let depth_image_memory_index = find_memorytype_index(
            &depth_image_memory_req,
            &device_memory_properties,
            vk::MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
        )?;

        let depth_image_allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MemoryAllocateInfo,
            p_next: ptr::null(),
            allocation_size: depth_image_memory_req.size,
            memory_type_index: depth_image_memory_index,
        };
        let depth_image_memory = device.allocate_memory(&depth_image_allocate_info, None)?;
        device.bind_image_memory(depth_image, depth_image_memory, 0)?;

        let depth_image_view_info = vk::ImageViewCreateInfo {
            s_type: vk::StructureType::ImageViewCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
            view_type: vk::ImageViewType::Type2d,
            format: depth_image_create_info.format,
            components: vk::ComponentMapping {
                r: vk::ComponentSwizzle::Identity,
                g: vk::ComponentSwizzle::Identity,
                b: vk::ComponentSwizzle::Identity,
                a: vk::ComponentSwizzle::Identity,
            },
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::IMAGE_ASPECT_DEPTH_BIT,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            image: depth_image,
        };

        let depth_image_view = device.create_image_view(&depth_image_view_info, None)?;

        self.depth_image_view = Some(depth_image_view);
        self.depth_image = Some(depth_image);

        Ok(self)
    }
}