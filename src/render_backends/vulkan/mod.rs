mod command_buffer;
mod debug_callback;
mod device;
mod framebuffers;
mod graphics_pipeline;
mod image_views;
mod instance;
mod physical_device;
mod render_pass;
mod semaphores;
mod surface;
mod swapchain;
mod vertex_buffer;

use ash::vk;
use ash::Entry;
use ash::Instance;
use ash::Device;
use ash::version::{V1_0, DeviceV1_0};
use ash::extensions::{Surface, Swapchain, DebugReport};
use winit;

use std::error::Error;
use std::u64;
use std::ptr;
use std::fmt;

use super::Vertex;
use super::Renderer;

type RendererDevice = Device<V1_0>;
type RendererInstance = Instance<V1_0>;
type RendererEntry = Entry<V1_0>;

pub struct VulkanRenderer {
    entry: Option<RendererEntry>,
    instance: Option<RendererInstance>,
    device: Option<RendererDevice>,

    window: Option<winit::Window>,
    window_width: Option<u64>,
    window_height: Option<u64>,

    debug_report_loader: Option<DebugReport>,
    debug_callback: Option<vk::DebugReportCallbackEXT>,

    physical_device: Option<vk::PhysicalDevice>,
    device_memory_properties: Option<vk::PhysicalDeviceMemoryProperties>,
    queue_family_index: Option<u32>,
    present_queue: Option<vk::Queue>,

    surface_loader: Option<Surface>,
    surface: Option<vk::SurfaceKHR>,
    surface_format: Option<vk::SurfaceFormatKHR>,
    surface_resolution: Option<vk::Extent2D>,

    swapchain_loader: Option<Swapchain>,
    swapchain: Option<vk::SwapchainKHR>,
    present_images: Option<Vec<vk::Image>>,
    present_image_views: Option<Vec<vk::ImageView>>,

    render_pass: Option<vk::RenderPass>,
    graphics_pipeline: Option<vk::Pipeline>,
    framebuffers: Option<Vec<vk::Framebuffer>>,
    vertex_buffer: Option<vk::Buffer>,

    command_pool: Option<vk::CommandPool>,
    command_buffers: Option<Vec<vk::CommandBuffer>>,

    depth_image: Option<vk::Image>,
    depth_image_view: Option<vk::ImageView>,
    depth_image_memory: Option<vk::DeviceMemory>,

    image_available_semaphore: Option<vk::Semaphore>,
    rendering_complete_semaphore: Option<vk::Semaphore>,
}

impl VulkanRenderer {
    pub fn new(
        window: &winit::Window,
        width: u32,
        height: u32,
    ) -> Result<VulkanRenderer, Box<Error>> {
        VulkanRenderer::builder()?
            .create_instance()?
            .set_debug_callback()?
            .create_surface_loader()?
            .create_surface()?
            .pick_physical_device()?
            .choose_surface_format()?
            .create_logical_device()?
            .create_swapchain()?
            .create_image_views()?
            .create_depth_view()?
            .create_render_pass()?
            .create_graphics_pipeline()?
            .create_framebuffers()?
            .create_command_pool()?
            //.create_vertex_buffer()?
            .create_command_buffers()?
            .create_semaphores()?
            .build()
    }

    fn builder() -> Result<VulkanRenderer, Box<Error>> {
        let entry = Entry::new().map_err(|_| "Couldn't create new entry")?;

        Ok(VulkanRenderer {
            entry: Some(entry),
            instance: None,
            device: None,

            window: None,
            window_width: None,
            window_height: None,

            debug_report_loader: None,
            debug_callback: None,

            physical_device: None,
            device_memory_properties: None,
            queue_family_index: None,
            present_queue: None,

            surface_loader: None,
            surface: None,
            surface_format: None,
            surface_resolution: None,

            swapchain_loader: None,
            swapchain: None,
            present_images: None,
            present_image_views: None,

            render_pass: None,
            graphics_pipeline: None,
            framebuffers: None,
            vertex_buffer: None,

            command_pool: None,
            command_buffers: None,

            depth_image: None,
            depth_image_view: None,
            depth_image_memory: None,

            image_available_semaphore: None,
            rendering_complete_semaphore: None,
        })
    }

    fn build(&mut self) -> Result<VulkanRenderer, Box<Error>> {
        Ok(*self)
    }
}

impl Renderer for VulkanRenderer {
    fn draw_vertices(&self, vertices: Vec<Vertex>) {}

    fn display_frame(&self) -> Result<(), Box<Error>> {
        let device = self.device.unwrap();
        let present_queue = self.present_queue.unwrap();
        let swapchain_loader = self.swapchain_loader.unwrap();
        let swapchain = self.swapchain.unwrap();
        let image_available_semaphore = self.image_available_semaphore.unwrap();
        let rendering_complete_semaphore = self.rendering_complete_semaphore.unwrap();
        let command_buffers = self.command_buffers.unwrap();

        let image_index = unsafe {
            device.queue_wait_idle(present_queue)?;

            swapchain_loader.acquire_next_image_khr(
                swapchain,
                u64::MAX,
                image_available_semaphore,
                vk::Fence::null(),
            )?
        };

        let wait_semaphores = [image_available_semaphore];
        let signal_semaphores = [rendering_complete_semaphore];
        let wait_mask = [vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT];

        let submit_info = vk::SubmitInfo {
            s_type: vk::StructureType::SubmitInfo,
            p_next: ptr::null(),
            wait_semaphore_count: wait_semaphores.len() as u32,
            p_wait_semaphores: wait_semaphores.as_ptr(),
            p_wait_dst_stage_mask: wait_mask.as_ptr(),
            command_buffer_count: 1,
            p_command_buffers: &command_buffers[image_index as usize],
            signal_semaphore_count: signal_semaphores.len() as u32,
            p_signal_semaphores: signal_semaphores.as_ptr(),
        };

        unsafe {
            device.queue_submit(
                present_queue,
                &[submit_info],
                vk::Fence::null(),
            )?;

            let present_info = vk::PresentInfoKHR {
                s_type: vk::StructureType::PresentInfoKhr,
                p_next: ptr::null(),
                wait_semaphore_count: 1,
                p_wait_semaphores: &rendering_complete_semaphore,
                swapchain_count: 1,
                p_swapchains: &swapchain,
                p_image_indices: &image_index,
                p_results: ptr::null_mut(),
            };
            swapchain_loader.queue_present_khr(present_queue, &present_info);
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

#[derive(Debug)]
pub enum RendererError {
    NoEntry,
    NoInstance,
    NoDevice,
    NoWindow,
    NoWidth,
    NoHeight,
    NoDebugReport,
    NoDebugReportCallbackEXT,
    NoPhysicalDevice,
    NoMemoryProperties,
    NoqueueFamilyIndex,
    NoPresentQueue,
    NoSurfaceLoader,
    NoSurface,
    NoSurfaceFormat,
    NoSurfaceResolution,
    NoExtent2D,
    NoSwapchainLoader,
    NoSwapchain,
    NoPresentImages,
    NoPresentImageViews,
    NoDepthImage,
    NoDepthImageView,
    NoDepthImageMemory,
    NoRenderPass,
    NoGraphicsPipeline,
    NoFramebuffers,
    NoVertexBuffer,
    NoCommandPool,
    NoCommandBuffers,
    NoSemaphore,
}

impl fmt::Display for RendererError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NoEntry => write!(f, "{}", "No entry specified"),
            NoInstance => write!(f, "{}", "No instance specified"),
            NoDevice => write!(f, "{}", "No device specified"),
            NoWindow => write!(f, "{}", "No window specified"),
            NoWidth => write!(f, "{}", "No width specified"),
            NoHeight => write!(f, "{}", "No height specified"),
            NoDebugReport => write!(f, "{}", "No debug report specified"),
            NoDebugReportCallback => write!(f, "{}", "No debug report callback specified"),
            NoPhysicalDevice => write!(f, "{}", "No physical device specified"),
            NoMemoryProperties => write!(f, "{}", "No physical device memory properties specified"),
            NoqueueFamilyIndex => write!(f, "{}", "No queue family index specified"),
            NoPresentQueue => write!(f, "{}", "No present queue specified"),
            NoSurfaceLoader => write!(f, "{}", "No surface loader specified"),
            NoSurface => write!(f, "{}", "No surface specified"),
            NoSurfaceFormat => write!(f, "{}", "No surface format specified"),
            NoSurfaceResolution => write!(f, "{}", "No surface resolution specified"),
            NoExtent2D => write!(f, "{}", "No 2D extent specified"),
            NoSwapchainLoader => write!(f, "{}", "No swapchain loader specified"),
            NoSwapchain => write!(f, "{}", "No swapchain specified"),
            NoPresentImages => write!(f, "{}", "No present images specified"),
            NoPresentImageViews => write!(f, "{}", "No present image views specified"),
            NoDepthImage => write!(f, "{}", "No depth image specified"),
            NoDepthImageView => write!(f, "{}", "No depth image view specified"),
            NoDepthImageMemory => write!(f, "{}", "No depth image memory specified"),
            NoRenderPass => write!(f, "{}", "No render pass specified"),
            NoGraphicsPipeline => write!(f, "{}", "No graphics pipeline specified"),
            NoFramebuffers => write!(f, "{}", "No framebuffers specified"),
            NoVertexBuffer => write!(f, "{}", "No vertex buffer specified"),
            NoCommandPool => write!(f, "{}", "No command pool specified"),
            NoCommandBuffers => write!(f, "{}", "No command buffers specified"),
            NoImageSemaphore => write!(f, "{}", "No image semaphore specified"),
            NoRenderingSemaphore => write!(f, "{}", "No rendering semaphore specified"),
        }
    }
}

impl Error for RendererError {
    fn description(&self) -> &str {
        format!("{}", self).as_str()
    }
}