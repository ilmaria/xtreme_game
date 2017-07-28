mod instance;
mod debug_callback;
mod surface;
mod physical_device;
mod device;
mod swapchain;
mod image_views;
mod framebuffers;
mod render_pass;
mod graphics_pipeline;
mod command_buffer;
mod semaphores;
mod vertex_buffer;

use ash::vk;
use ash::Entry;
use ash::Instance;
use ash::Device;
use ash::version::{V1_0, DeviceV1_0};
use ash::extensions::Surface;
use winit;

use std::error::Error;
use std::u64;
use std::ptr;

use self::instance::create_instance;
use self::debug_callback::set_debug_callback;
use self::surface::create_surface;
use self::physical_device::pick_physical_device;
use self::device::create_logical_device;
use self::swapchain::create_swapchain;
use self::image_views::{create_image_views, create_depth_view};
use self::render_pass::create_render_pass;
use self::graphics_pipeline::create_graphics_pipeline;
use self::framebuffers::create_framebuffers;
use self::command_buffer::create_command_buffers;
use self::command_buffer::create_command_pool;
use self::vertex_buffer::create_vertex_buffer;

pub use super::graphics::Vertex;

type DeviceV10 = Device<V1_0>;
type InstanceV10 = Instance<V1_0>;
type EntryV10 = Entry<V1_0>;

pub struct RenderState {}

pub struct SurfaceDetails {
    khr: vk::SurfaceKHR,
    loader: Surface,
    format: vk::SurfaceFormatKHR,
    capabilities: vk::SurfaceCapabilitiesKHR,
    resolution: vk::Extent2D,
}

pub fn init_vulkan(window: &winit::Window, width: u32, height: u32) -> Result<(), Box<Error>> {
    let entry = Entry::new().map_err(|_| "Couldn't create new entry")?;
    let instance = create_instance(&entry)?;
    set_debug_callback(&entry, &instance)?;
    let surface = create_surface(&entry, &instance, window, width, height)?;
    let (physical_device, queue_family_index) =
        pick_physical_device(&instance, &surface.khr, &surface.loader)?;
    let device = create_logical_device(physical_device, &instance, queue_family_index)?;
    let queue = unsafe { device.get_device_queue(queue_family_index, 0) };
    let (swapchain, swapchain_loader) =
        create_swapchain(&device, &instance, physical_device, surface)?;
    let present_image_views = create_image_views(&device, &surface, swapchain, swapchain_loader)?;
    let depth_view = create_depth_view(&device, &instance, physical_device, &surface)?;
    let render_pass = create_render_pass(&device, &surface)?;
    let graphics_pipeline = create_graphics_pipeline(&device, &surface, render_pass);
    let framebuffers = create_framebuffers(
        &device,
        present_image_views,
        depth_view,
        render_pass,
        &surface,
    )?;
    let command_pool = create_command_pool(&device, queue_family_index)?;
    let command_buffers = create_command_buffers(&device, command_pool, framebuffers.len() as u32)?;
    Ok(())
}

// pub fn render() -> Result<(), Box<Error>> {
//     device.queue_wait_idle(present_queue)?;

//     let image_index = swapchain_loader.acquire_next_image_khr(
//         swapchain,
//         u64::MAX,
//         image_available_semaphore,
//         vk::Fence::null(),
//     )?;

//     let wait_semaphores = [image_available_semaphore];
//     let signal_semaphores = [rendering_complete_semaphore];
//     let wait_mask = [vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT];

//     let submit_info = vk::SubmitInfo {
//         s_type: vk::StructureType::SubmitInfo,
//         p_next: ptr::null(),
//         wait_semaphore_count: wait_semaphores.len() as u32,
//         p_wait_semaphores: wait_semaphores.as_ptr(),
//         p_wait_dst_stage_mask: wait_mask.as_ptr(),
//         command_buffer_count: 1,
//         p_command_buffers: &command_buffers[image_index],
//         signal_semaphore_count: signal_semaphores.len() as u32,
//         p_signal_semaphores: signal_semaphores.as_ptr(),
//     };

//     device.queue_submit(
//         present_queue,
//         &[submit_info],
//         ptr::null(),
//     )?;

//     let present_info = vk::PresentInfoKHR {
//         s_type: vk::StructureType::PresentInfoKhr,
//         p_next: ptr::null(),
//         wait_semaphore_count: 1,
//         p_wait_semaphores: &rendering_complete_semaphore,
//         swapchain_count: 1,
//         p_swapchains: &swapchain,
//         p_image_indices: &image_index,
//         p_results: ptr::null_mut(),
//     };
//     swapchain_loader.queue_present_khr(present_queue, &present_info);

//     Ok(())
// }

// pub fn recreate_swapchain() -> Result<(), Box<Error>> {
//     let (swapchain, swapchain_loader) =
//         create_swapchain(&device, &instance, physical_device, surface)?;
//     let present_image_views = create_image_views(&device, &surface, swapchain, swapchain_loader)?;
//     let depth_view = create_depth_view(&device, &instance, physical_device, &surface)?;
//     let render_pass = create_render_pass(&device, &surface)?;
//     let graphics_pipeline = create_pipeline(&device, &surface, render_pass);
//     let framebuffers = create_framebuffers(
//         &device,
//         present_image_views,
//         depth_view,
//         render_pass,
//         &surface,
//     )?;
//     let command_pool = create_command_pool(&device, queue_family_index)?;
//     let command_buffers = create_command_buffers(&device, command_pool, framebuffers.len() as u32)?;

//     Ok(())
// }