mod command_buffer;
mod command_pool;
mod debug_callback;
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
mod vertex_buffer;

use ash::vk;
use ash::Entry;
use ash::Instance;
use ash::Device;
use ash::version::{V1_0, DeviceV1_0};
use ash::extensions::{Surface, Swapchain};
use winit;

use std::error::Error;
use std::u64;
use std::ptr;

use super::Vertex;
use super::Renderer;

pub struct VulkanRenderer {
    entry: Entry<V1_0>,
    instance: Instance<V1_0>,
    device: Device<V1_0>,

    physical_device: vk::PhysicalDevice,
    queue_family_index: u32,
    present_queue: vk::Queue,

    surface_loader: Surface,
    surface: vk::SurfaceKHR,
    surface_format: vk::SurfaceFormatKHR,
    surface_resolution: vk::Extent2D,

    swapchain_loader: Swapchain,
    swapchain: vk::SwapchainKHR,
    present_image_views: Vec<vk::ImageView>,

    current_swapchain_index: u32,
    swapchain_fences: Vec<vk::Fence>,
    swapchain_semaphores: Vec<vk::Semaphore>,

    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,

    depth_image: vk::Image,
    depth_image_view: vk::ImageView,
    depth_image_memory: vk::DeviceMemory,
}

impl VulkanRenderer {
    pub fn new(
        window: &winit::Window,
        width: u32,
        height: u32,
    ) -> Result<VulkanRenderer, Box<Error>> {
        let entry = Entry::new().map_err(|_| "Couldn't create new entry")?;

        let instance = instance::new(&entry)?;

        debug_callback::new(&entry, &instance)?;

        let surface_loader = surface::new_loader(&entry, &instance)?;

        let surface = surface::new(&entry, &instance, window)?;

        let (physical_device, queue_family_index) =
            physical_device::new_with_queue(&instance, surface, &surface_loader)?;

        let surface_format = surface::new_format(physical_device, surface, &surface_loader)?;
        let surface_resolution =
            surface::new_resolution(physical_device, surface, &surface_loader, width, height)?;

        let device = device::new(&instance, queue_family_index, physical_device)?;

        let present_queue = unsafe { device.get_device_queue(queue_family_index as u32, 0) };

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

        let command_pool = command_pool::new(&device, queue_family_index)?;

        let command_buffers = command_buffer::new(&device, command_pool, swapchain_len as u32)?;

        let swapchain_fences = swapchain::new_fences(&device, swapchain_len)?;
        let swapchain_semaphores = swapchain::new_semaphores(&device, swapchain_len)?;

        Ok(VulkanRenderer {
            entry,
            instance,
            device,

            physical_device,
            queue_family_index,
            present_queue,

            surface_loader,
            surface,
            surface_format,
            surface_resolution,

            swapchain_loader,
            swapchain,
            present_image_views,

            current_swapchain_index: 0,
            swapchain_fences,
            swapchain_semaphores,

            command_pool,
            command_buffers,

            depth_image,
            depth_image_view,
            depth_image_memory,
        })
    }

    #[must_use]
    unsafe fn acquire_next_image(&mut self) -> Result<(), Box<Error>> {
        let fence = {
            let fence_info = vk::FenceCreateInfo {
                s_type: vk::StructureType::FenceCreateInfo,
                p_next: ptr::null(),
                flags: Default::default(),
            };

            self.device.create_fence(&fence_info, None)?
        };

        let index = {
            self.swapchain_loader.acquire_next_image_khr(
                self.swapchain,
                u64::MAX,
                vk::Semaphore::null(),
                fence,
            )?
        };

        self.current_swapchain_index = index;

        self.device.wait_for_fences(&[fence], true, u64::MAX)?;
        self.device.destroy_fence(fence, None);

        self.device.wait_for_fences(
            &[self.swapchain_fences[index as usize]],
            true,
            u64::MAX,
        )?;

        Ok(())
    }
}

impl Renderer for VulkanRenderer {
    fn draw_vertices(&self, vertices: Vec<Vertex>) {}

    #[must_use]
    fn display_frame(&self) -> Result<(), Box<Error>> {
        let submit_info = vk::SubmitInfo {
            s_type: vk::StructureType::SubmitInfo,
            p_next: ptr::null(),
            wait_semaphore_count: 0,
            p_wait_semaphores: [].as_ptr(),
            p_wait_dst_stage_mask: [].as_ptr(),
            command_buffer_count: 1,
            p_command_buffers: &self.command_buffers[self.current_swapchain_index as usize],
            signal_semaphore_count: self.swapchain_semaphores.len() as u32,
            p_signal_semaphores: self.swapchain_semaphores.as_ptr(),
        };

        unsafe {
            self.device.queue_submit(
                self.present_queue,
                &[submit_info],
                vk::Fence::null(),
            )?;

            let present_info = vk::PresentInfoKHR {
                s_type: vk::StructureType::PresentInfoKhr,
                p_next: ptr::null(),
                wait_semaphore_count: 1,
                p_wait_semaphores: self.swapchain_semaphores.as_ptr(),
                swapchain_count: 1,
                p_swapchains: &self.swapchain,
                p_image_indices: &self.current_swapchain_index,
                p_results: ptr::null_mut(),
            };
            self.swapchain_loader.queue_present_khr(
                self.present_queue,
                &present_info,
            )?;
        }

        Ok(())
    }

    fn update_resolution(&self, width: u64, height: u64) {}

    fn change_settings(&self) {}
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
    Err(
        "Unable to find suitable memory index for depth image.".to_owned(),
    )
}
