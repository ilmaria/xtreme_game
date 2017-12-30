mod command;
mod device;
mod framebuffers;
mod graphics_pipeline;
mod image_views;
mod instance;
mod physical_device;
mod render_pass;
mod semaphore;
mod surface;
mod swapchain;
mod buffer;

use ash::vk;
use ash::Entry;
use ash::Instance;
use ash::Device;
use ash::version::{DeviceV1_0, V1_0};
use ash::extensions as ext;
use winit;

use std::error::Error;
use std::u64;
use std::ptr;
use std::ffi::CStr;

use self::surface::Surface;

lazy_static! {
    pub static ref VK_ENTRY: Entry<V1_0> = Entry::new().unwrap();
    pub static ref VK_INSTANCE: Instance<V1_0> = instance::new(&VK_ENTRY).unwrap();
}

type VkDevice = Device<V1_0>;

pub struct Renderer {
    device: VkDevice,

    physical_device: vk::PhysicalDevice,
    queue_family_index: u32,
    graphics_queue: vk::Queue,

    surface_loader: Surface,
    surface: vk::SurfaceKHR,
    surface_format: vk::SurfaceFormatKHR,
    surface_resolution: vk::Extent2D,

    frame_index: u32,
    swapchain_loader: Swapchain,
    swapchain: vk::SwapchainKHR,
    swapchain_frames: Vec<swapchain::Frame>,

    command_pool: vk::CommandPool,

    depth_image: vk::Image,
    depth_image_view: vk::ImageView,
    depth_image_memory: vk::DeviceMemory,

    allocator: buffer::Allocator,
    staging_buffer: usize,
    vertex_buffer: usize,
}

impl Renderer {
    pub fn new(window: &winit::Window) -> Result<Renderer, Box<Error>> {
        set_debug_callback()?;

        let surface = Surface::new(window)?;

        let (physical_device, queue_family_index) =
            physical_device::new_with_queue(&instance, surface, &surface_loader)?;

        let surface_format = surface::new_format(physical_device, surface, &surface_loader)?;
        let surface_resolution =
            surface::new_resolution(physical_device, surface, &surface_loader, width, height)?;

        let device = device::new(&instance, queue_family_index, physical_device)?;

        let graphics_queue = unsafe { device.get_device_queue(queue_family_index as u32, 0) };

        let swapchain_loader = swapchain::new_loader(&device, &instance)?;

        let swapchain = swapchain::new(
            &swapchain_loader,
            physical_device,
            surface,
            &surface_loader,
            &surface_format,
            &surface_resolution,
        )?;

        let (depth_image, depth_image_memory) =
            image_views::new_depth_image(&device, &instance, &surface_resolution, physical_device)?;
        let depth_image_view = image_views::new_depth_view(&device, depth_image)?;

        let present_image_views =
            image_views::new(&device, swapchain, &swapchain_loader, &surface_format)?;

        let swapchain_len = present_image_views.len();

        let render_pass = render_pass::new(&device, &surface_format)?;

        let graphics_pipeline =
            graphics_pipeline::new(&device, render_pass, &surface_resolution, depth_image_view)?;

        let framebuffers = framebuffers::new(
            &device,
            render_pass,
            &surface_resolution,
            &present_image_views,
            depth_image_view,
        )?;

        let command_pool = command::new_pool(&device, queue_family_index)?;

        let command_buffers = command::new_buffers(&device, command_pool, swapchain_len as u32)?;

        let swapchain_frames = {
            let mut frames = vec![];
            for i in 0..swapchain_len {
                frames.push(swapchain::Frame::new(
                    &device,
                    present_image_views[i],
                    framebuffers[i],
                    command_buffers[i],
                )?);
            }
            frames
        };

        command::submit(
            &device,
            swapchain_frames[0].command_buffer,
            graphics_queue,
            &[vk::PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT],
            &[],
            &[],
            |device, setup_command_buffer| {
                let layout_transition_barrier = vk::ImageMemoryBarrier {
                    s_type: vk::StructureType::ImageMemoryBarrier,
                    p_next: ptr::null(),
                    src_access_mask: Default::default(),
                    dst_access_mask: vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT
                        | vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
                    old_layout: vk::ImageLayout::Undefined,
                    new_layout: vk::ImageLayout::DepthStencilAttachmentOptimal,
                    src_queue_family_index: vk::VK_QUEUE_FAMILY_IGNORED,
                    dst_queue_family_index: vk::VK_QUEUE_FAMILY_IGNORED,
                    image: depth_image,
                    subresource_range: vk::ImageSubresourceRange {
                        aspect_mask: vk::IMAGE_ASPECT_DEPTH_BIT,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                };

                unsafe {
                    device.cmd_pipeline_barrier(
                        setup_command_buffer,
                        vk::PIPELINE_STAGE_TOP_OF_PIPE_BIT,
                        vk::PIPELINE_STAGE_TOP_OF_PIPE_BIT,
                        vk::DependencyFlags::empty(),
                        &[],
                        &[],
                        &[layout_transition_barrier],
                    )
                };
            },
        )?;

        for (i, &command_buffer) in command_buffers.iter().enumerate() {
            let clear_values = [
                vk::ClearValue::new_color(vk::ClearColorValue::new_float32([0.0, 0.0, 0.0, 0.0])),
                vk::ClearValue::new_depth_stencil(vk::ClearDepthStencilValue {
                    depth: 1.0,
                    stencil: 0,
                }),
            ];
            let render_pass_info = vk::RenderPassBeginInfo {
                s_type: vk::StructureType::RenderPassBeginInfo,
                render_pass,
                framebuffer: framebuffers[i],
                render_area: vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: surface_resolution.clone(),
                },
                clear_value_count: clear_values.len() as u32,
                p_clear_values: clear_values.as_ptr(),
                p_next: ptr::null(),
            };

            let viewports = [
                vk::Viewport {
                    x: 0.0,
                    y: 0.0,
                    width: surface_resolution.width as f32,
                    height: surface_resolution.height as f32,
                    min_depth: 0.0,
                    max_depth: 1.0,
                },
            ];
            let scissors = [
                vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: surface_resolution.clone(),
                },
            ];

            unsafe {
                command::submit(
                    &device,
                    command_buffer,
                    graphics_queue,
                    &[vk::PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT],
                    &[],
                    &[],
                    |device, cmd| {
                        &device.cmd_bind_pipeline(
                            cmd,
                            vk::PipelineBindPoint::Graphics,
                            graphics_pipeline,
                        );
                        &device.cmd_begin_render_pass(
                            cmd,
                            &render_pass_info,
                            vk::SubpassContents::Inline,
                        );
                        device.cmd_set_viewport(cmd, &viewports);
                        device.cmd_set_scissor(cmd, &scissors);
                    },
                )?;
            };
        }

        let mut allocator = buffer::Allocator::new(physical_device);

        let staging_buffer = allocator.create_buffer(
            &device,
            &instance,
            4048,
            vk::BUFFER_USAGE_TRANSFER_SRC_BIT,
            vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk::MEMORY_PROPERTY_HOST_COHERENT_BIT,
        )?;

        let vertex_buffer = allocator.create_buffer(
            &device,
            &instance,
            4048,
            vk::BUFFER_USAGE_TRANSFER_DST_BIT | vk::BUFFER_USAGE_VERTEX_BUFFER_BIT,
            vk::MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
        )?;

        Ok(VulkanRenderer {
            entry,
            instance,
            device,

            physical_device,
            queue_family_index,
            graphics_queue,

            surface_loader,
            surface,
            surface_format,
            surface_resolution,

            swapchain_loader,
            swapchain,
            swapchain_frames,
            frame_index: 0,

            command_pool,

            depth_image,
            depth_image_view,
            depth_image_memory,

            allocator,
            staging_buffer,
            vertex_buffer,
        })
    }
}

pub fn find_memorytype_index(
    memory_req: &vk::MemoryRequirements,
    memory_prop: &vk::PhysicalDeviceMemoryProperties,
    flags: vk::MemoryPropertyFlags,
) -> Result<u32, String> {
    // Try to find an exactly matching memory flag
    let best_suitable_index =
        find_memorytype_index_f(memory_req, memory_prop, flags, |property_flags, flags| {
            property_flags == flags
        });
    if best_suitable_index.is_ok() {
        return best_suitable_index;
    }
    // Otherwise find a memory flag that works
    find_memorytype_index_f(memory_req, memory_prop, flags, |property_flags, flags| {
        property_flags & flags == flags
    })
}

fn find_memorytype_index_f<F: Fn(vk::MemoryPropertyFlags, vk::MemoryPropertyFlags) -> bool>(
    memory_req: &vk::MemoryRequirements,
    memory_prop: &vk::PhysicalDeviceMemoryProperties,
    flags: vk::MemoryPropertyFlags,
    f: F,
) -> Result<u32, String> {
    let mut memory_type_bits = memory_req.memory_type_bits;
    for (index, ref memory_type) in memory_prop.memory_types.iter().enumerate() {
        if memory_type_bits & 1 == 1 {
            if f(memory_type.property_flags, flags) {
                return Ok(index as u32);
            }
        }
        memory_type_bits = memory_type_bits >> 1;
    }
    Err("Unable to find suitable memory index for depth image.".to_owned())
}

pub fn set_debug_callback() -> Result<(), Box<Error>> {
    let debug_info = vk::DebugReportCallbackCreateInfoEXT {
        s_type: vk::StructureType::DebugReportCallbackCreateInfoExt,
        p_next: ptr::null(),
        flags: vk::DEBUG_REPORT_ERROR_BIT_EXT | vk::DEBUG_REPORT_WARNING_BIT_EXT
            | vk::DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT,
        pfn_callback: vulkan_debug_callback,
        p_user_data: ptr::null_mut(),
    };

    let debug_report_loader = ext::DebugReport::new(VK_ENTRY, VK_INSTANCE)
        .map_err(|_| "Couldn't create debug repoprt loader")?;

    let callback =
        unsafe { debug_report_loader.create_debug_report_callback_ext(&debug_info, None)? };

    Ok(())
}

unsafe extern "system" fn vulkan_debug_callback(
    _: vk::DebugReportFlagsEXT,
    _: vk::DebugReportObjectTypeEXT,
    _: vk::uint64_t,
    _: vk::size_t,
    _: vk::int32_t,
    _: *const vk::c_char,
    p_message: *const vk::c_char,
    _: *mut vk::c_void,
) -> u32 {
    println!("{:?}", CStr::from_ptr(p_message));
    1
}
