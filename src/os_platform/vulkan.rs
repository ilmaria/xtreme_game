use ash::vk;
use ash::Entry;
use ash::Instance;
use ash::Device;
use ash::version::{V1_0, InstanceV1_0, DeviceV1_0, EntryV1_0};
use ash::extensions::{Swapchain, XlibSurface, Surface, DebugReport, Win32Surface};
use winit;
use glsl_to_spirv;

use std::ptr;
use std::u32;
use std::default::Default;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::ffi::CString;

pub fn init_vulkan(window_width: u32, window_height: u32) -> Result<(), Box<Error>> {
    let entry = Entry::new()?;
    let instance = create_instance(entry)?;
    set_debug_callback(&entry, &instance)?;
    let surface = create_surface(&entry, &instance)?;
    let surface_loader = Surface::new(&entry, &instance)?;
    let (physical_device, queue_family_index) = pick_physical_device(&instance, surface, surface_loader)?;
    let device = create_logical_device(physical_device, queue_family_index)?;
    let queue = device.get_device_queue(queue_family_index, 0);
    let swapchain = create_swapchain(&device, &instance, physical_device, surface, surface_loader)?;
}

fn create_instance(entry: &Entry<V1_0>) -> Result<Instance<V1_0>, Box<Error>> {
    let app_name = CString::new("Xtreme Game")?.as_ptr();

    let appinfo = vk::ApplicationInfo {
        p_application_name: app_name,
        s_type: vk::StructureType::ApplicationInfo,
        p_next: ptr::null(),
        application_version: 0,
        p_engine_name: app_name,
        engine_version: 0,
        api_version: vk_make_version!(1, 0, 36),
    };

    let layer_names = [CString::new("VK_LAYER_LUNARG_standard_validation")?];
    let layers_names_raw: Vec<*const i8> = layer_names
        .iter()
        .map(|raw_name| raw_name.as_ptr())
        .collect();

    let os_surface = if cfg!(windows) {
        Win32Surface::name()
    } else /*if cfg!(all(unix, not(target_os = "android")))*/ {
        XlibSurface::name()
    };

    let extension_names = vec![
        Surface::name().as_ptr(),
        os_surface.as_ptr(),
        DebugReport::name().as_ptr(),
    ];

    let create_info = vk::InstanceCreateInfo {
        s_type: vk::StructureType::InstanceCreateInfo,
        p_next: ptr::null(),
        flags: Default::default(),
        p_application_info: &appinfo,
        pp_enabled_layer_names: layers_names_raw.as_ptr(),
        enabled_layer_count: layers_names_raw.len() as u32,
        pp_enabled_extension_names: extension_names.as_ptr(),
        enabled_extension_count: extension_names.len() as u32,
    };

    let instance = entry.create_instance(&create_info, None)?;
    Ok(instance)
}

fn set_debug_callback(entry: &Entry<V1_0>, instance: &Instance<V1_0>) -> Result<vk::DebugReportCallbackEXT, Box<Error>>{
    let debug_info = vk::DebugReportCallbackCreateInfoEXT {
        s_type: vk::StructureType::DebugReportCallbackCreateInfoExt,
        p_next: ptr::null(),
        flags: vk::DEBUG_REPORT_ERROR_BIT_EXT |
               vk::DEBUG_REPORT_WARNING_BIT_EXT |
               vk::DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT,
        pfn_callback: vulkan_debug_callback,
        p_user_data: ptr::null_mut(),
    };
    let debug_report_loader = DebugReport::new(entry, instance).unwrap();
    let callback = debug_report_loader.create_debug_report_callback_ext(&debug_info, None)?;
    Ok(callback)
}

#[cfg(all(unix, not(target_os = "android")))]
unsafe fn create_surface(entry: &Entry<V1_0>,
                  instance: &Instance<V1_0>,
                  window: &winit::Window)
                  -> Result<vk::SurfaceKHR, Box<Error>> {
    use winit::os::unix::WindowExt;
    let x11_display = window.get_xlib_display().unwrap();
    let x11_window = window.get_xlib_window().unwrap();
    let x11_create_info = vk::XlibSurfaceCreateInfoKHR {
        s_type: vk::StructureType::XlibSurfaceCreateInfoKhr,
        p_next: ptr::null(),
        flags: Default::default(),
        window: x11_window as vk::Window,
        dpy: x11_display as *mut vk::Display,
    };
    let xlib_surface_loader = XlibSurface::new(entry, instance)
        .expect("Unable to load xlib surface");
    let surface = xlib_surface_loader.create_xlib_surface_khr(&x11_create_info, None)?;
    Ok(surface)
}

#[cfg(windows)]
unsafe fn create_surface(entry: &Entry<V1_0>,
                  instance: &Instance<V1_0>,
                  window: &winit::Window)
                  -> Result<vk::SurfaceKHR, Box<Error>> {
    use winit::os::windows::WindowExt;
    let hwnd = window.get_hwnd();
    let win32_create_info = vk::Win32SurfaceCreateInfoKHR {
        s_type: vk::StructureType::Win32SurfaceCreateInfoKhr,
        p_next: ptr::null(),
        flags: Default::default(),
        hinstance: ptr::null(),
        hwnd: hwnd as *const (),
    };
    let win32_surface_loader = Win32Surface::new(entry, instance)?;
    win32_surface_loader.create_win32_surface_khr(&win32_create_info, None)
}

fn pick_physical_device(instance: &Instance<V1_0>,
                        surface: vk::SurfaceKHR,
                        surface_loader: Surface)
                        -> Result<(vk::PhysicalDevice, usize), Box<Error>> {
    let device = instance
        .enumerate_physical_devices()?
        .iter()
        .map(|device| {
            instance.get_physical_device_queue_family_properties(*device)
                .iter()
                .enumerate()
                .filter_map(|(index, ref info)| {
                    let supports_graphic_and_surface =
                        info.queue_flags.subset(vk::QUEUE_GRAPHICS_BIT) &&
                        surface_loader.get_physical_device_surface_support_khr(*device, index as u32, surface);
                    match supports_graphic_and_surface {
                        true => Some((*device, index)),
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

fn create_logical_device(physical_device: vk::types::PhysicalDevice,
                         instance: Instance<V1_0>,
                         queue_family_index: u32)
                         -> Result<Device<V1_0>, Box<Error>> {
    let device_extension_names = [Swapchain::name().as_ptr()];
    let features = vk::PhysicalDeviceFeatures {
        shader_clip_distance: 1,
        ..Default::default()
    };
    let priorities = [1.0];
    let queue_info = vk::DeviceQueueCreateInfo {
        s_type: vk::StructureType::DeviceQueueCreateInfo,
        p_next: ptr::null(),
        flags: Default::default(),
        queue_family_index: queue_family_index,
        p_queue_priorities: priorities.as_ptr(),
        queue_count: priorities.len() as u32,
    };
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

    instance.create_device(physical_device, &device_create_info, None)
}

fn create_swapchain(device: &Device<V1_0>,
                    instance: &Instance<V1_0>,
                    window_width: u32,
                    window_height: u32,
                    physical_device: vk::types::PhysicalDevice,
                    surface: vk::SurfaceKHR,
                    surface_loader: Surface)
                    -> Result<vk::SwapchainKHR, Box<Error>> {
    let surface_format = surface_loader
        .get_physical_device_surface_formats_khr(physical_device, surface)?
        .iter()
        .map(|sfmt| {
            match sfmt.format {
                vk::Format::Undefined => {
                    vk::SurfaceFormatKHR {
                        format: vk::Format::B8g8r8Unorm,
                        color_space: sfmt.color_space,
                    }
                }
                _ => sfmt.clone(),
            }
        })
        .nth(0)
        .unwrap();

    let surface_capabilities = surface_loader
        .get_physical_device_surface_capabilities_khr(physical_device, surface)?;
    let mut desired_image_count = surface_capabilities.min_image_count + 1;

    if surface_capabilities.max_image_count > 0 &&
        desired_image_count > surface_capabilities.max_image_count {
        desired_image_count = surface_capabilities.max_image_count;
    }
    let surface_resolution =
        match surface_capabilities.current_extent.width {
            u32::MAX => {
                vk::Extent2D {
                    width: window_width,
                    height: window_height,
                }
            }
            _ => surface_capabilities.current_extent,
        };

    let pre_transform = if surface_capabilities.supported_transforms
        .subset(vk::SURFACE_TRANSFORM_IDENTITY_BIT_KHR) {
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

    let swapchain_loader = Swapchain::new(instance, device)
        .expect("Unable to load swapchain");
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

    swapchain_loader.create_swapchain_khr(&swapchain_create_info, None)
}

fn create_image_views(device: &Device<V1_0>,
                      surface_format: vk::SurfaceFormatKHR,
                      swapchain: vk::SwapchainKHR,
                      swapchain_loader: Swapchain)
                      -> Result<Vec<vk::ImageView>, Box<Error>> {
    let image_views: Vec<vk::ImageView> = swapchain_loader
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
            device.create_image_view(&create_view_info, None).unwrap()
        })
        .collect();

    Ok(image_views)
}

fn create_depth_view(device: &Device<V1_0>,
                      instance: &Instance<V1_0>,
                      physical_device: vk::types::PhysicalDevice,
                      surface_format: vk::SurfaceFormatKHR,
                      surface_resolution: vk::types::Extent2D)
                      -> Result<vk::ImageView, Box<Error>> {
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

    let device_memory_properties = instance.get_physical_device_memory_properties(physical_device);
    let depth_image_memory_req = device.get_image_memory_requirements(depth_image);
    let depth_image_memory_index = find_memorytype_index(
        &depth_image_memory_req,
        &device_memory_properties,
        vk::MEMORY_PROPERTY_DEVICE_LOCAL_BIT
    ).expect("Unable to find suitable memory index for depth image.");

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

    device.create_image_view(&depth_image_view_info, None)?
}

fn create_pipeline(device: &Device<V1_0>) -> Result<vk::types::ShaderModule, Box<Error>> {
    let vert_shader = create_shader_module(&device, "data/shaders/cube.vert", glsl_to_spirv::ShaderType::Vertex)?;
    let frag_shader = create_shader_module(&device, "data/shaders/cube.frag", glsl_to_spirv::ShaderType::Fragment)?;

    let entry_name = CString::new("main")?;
    let vert_shader_stage_info = vk::PipelineShaderStageCreateInfo {
        s_type: vk::StructureType::PipelineShaderStageCreateInfo,
        p_next: ptr::null(),
        flags: Default::default(),
        module: vert_shader,
        p_name: entry_name.as_ptr(),
        p_specialization_info: ptr::null(),
        stage: vk::SHADER_STAGE_VERTEX_BIT,
    };
    let frag_shader_stage_info = vk::PipelineShaderStageCreateInfo {
        s_type: vk::StructureType::PipelineShaderStageCreateInfo,
        p_next: ptr::null(),
        flags: Default::default(),
        module: frag_shader,
        p_name: entry_name.as_ptr(),
        p_specialization_info: ptr::null(),
        stage: vk::SHADER_STAGE_FRAGMENT_BIT,
    };
}

fn create_shader_module(device: &Device<V1_0>, path: Path, shader_type: glsl_to_spirv::ShaderType) -> Result<vk::types::ShaderModule, Box<Error>> {
    let shader_bytes = {
        let shader_file = File::open(path);
        let mut bytes = Vec::new();
        shader_file.read_to_end(&mut bytes)?;
        bytes
    };

    let spv_file = glsl_to_spirv::compile(shader_bytes, shader_type)?;

    let spv_bytes: Vec<u8> = spv_file
        .bytes()
        .filter_map(|byte| byte.ok())
        .collect();

    let shader_info = vk::ShaderModuleCreateInfo {
        s_type: vk::StructureType::ShaderModuleCreateInfo,
        p_next: ptr::null(),
        flags: Default::default(),
        code_size: spv_bytes.len(),
        p_code: spv_bytes.as_ptr() as *const u32,
    };

    device.create_shader_module(&shader_info, None)
}

fn find_memorytype_index(memory_req: &vk::MemoryRequirements,
                         memory_prop: &vk::PhysicalDeviceMemoryProperties,
                         flags: vk::MemoryPropertyFlags)
                         -> Option<u32> {
    // Try to find an exactly matching memory flag
    let best_suitable_index =
        find_memorytype_index_f(memory_req,
                                memory_prop,
                                flags,
                                |property_flags, flags| property_flags == flags);
    if best_suitable_index.is_some() {
        return best_suitable_index;
    }
    // Otherwise find a memory flag that works
    find_memorytype_index_f(memory_req,
                            memory_prop,
                            flags,
                            |property_flags, flags| property_flags & flags == flags)
}

fn find_memorytype_index_f<F: Fn(vk::MemoryPropertyFlags, vk::MemoryPropertyFlags) -> bool>
    (memory_req: &vk::MemoryRequirements,
     memory_prop: &vk::PhysicalDeviceMemoryProperties,
     flags: vk::MemoryPropertyFlags,
     f: F)
     -> Option<u32> {
    let mut memory_type_bits = memory_req.memory_type_bits;
    for (index, ref memory_type) in memory_prop.memory_types.iter().enumerate() {
        if memory_type_bits & 1 == 1 {
            if f(memory_type.property_flags, flags) {
                return Some(index as u32);
            }
        }
        memory_type_bits = memory_type_bits >> 1;
    }
    None
}

unsafe extern "system" fn vulkan_debug_callback(
        _: vk::DebugReportFlagsEXT,
        _: vk::DebugReportObjectTypeEXT,
        _: vk::uint64_t,
        _: vk::size_t,
        _: vk::int32_t,
        _: *const vk::c_char,
        p_message: *const vk::c_char,
        _: *mut vk::c_void)
    -> u32 {
    println!("{:?}", CStr::from_ptr(p_message));
    1
}