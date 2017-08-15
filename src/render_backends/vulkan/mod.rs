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
mod buffer;

use ash::vk;
use ash::Entry;
use ash::Instance;
use ash::Device;
use ash::version::{DeviceV1_0, V1_0};
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

    staging_buffer: buffer::Buffer,
    vertex_buffer: buffer::Buffer,
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

        let command_pool = command_pool::new(&device, queue_family_index)?;

        let command_buffers = command_buffer::new(&device, command_pool, swapchain_len as u32)?;

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

        command_buffer::submit(&device,
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
                dst_access_mask: vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT |
                                    vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
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
            device.cmd_pipeline_barrier(setup_command_buffer,
                                        vk::PIPELINE_STAGE_TOP_OF_PIPE_BIT,
                                        vk::PIPELINE_STAGE_TOP_OF_PIPE_BIT,
                                        vk::DependencyFlags::empty(),
                                        &[],
                                        &[],
                                        &[layout_transition_barrier])};
        })?;

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
                command_buffer::submit(
                    &device,
                    command_buffer,
                    graphics_queue,
                    &[vk::PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT],
                    &[],
                    &[],
                    |device, cmd| {
                        &device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::Graphics, graphics_pipeline);
                        &device.cmd_begin_render_pass(cmd, &render_pass_info, vk::SubpassContents::Inline);
                        device.cmd_set_viewport(cmd, &viewports);
                        device.cmd_set_scissor(cmd, &scissors);
                })?;
            };
        }

        let staging_buffer = buffer::Buffer::new(
            &device,
            &instance,
            physical_device,
            4048,
            vk::BUFFER_USAGE_TRANSFER_SRC_BIT,
            vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk::MEMORY_PROPERTY_HOST_COHERENT_BIT,
        )?;

        let vertex_buffer = buffer::Buffer::new(
            &device,
            &instance,
            physical_device,
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

            staging_buffer,
            vertex_buffer,
        })
    }
}

impl Renderer for VulkanRenderer {
    #[must_use]
    fn load_vertices(&self, vertices: Vec<Vertex>) -> Result<(), Box<Error>> {
        // buffer::Buffer::copy_vertices_to_device(
        //     &self.device,
        //     &self.instance,
        //     self.physical_device,
        //     self.command_pool,
        //     self.graphics_queue,
        //     &self.staging_buffer,
        //     &self.vertex_buffer,
        //     vertices,
        // )?;

        // for ref frame in self.swapchain_frames.iter() {
        //     let cmd = frame.command_buffer;

        //     unsafe {
        //         &self.device
        //             .cmd_bind_vertex_buffers(cmd, 0, &[self.vertex_buffer.buf], &[0]);
        //     };
        // }

        Ok(())
    }

    #[must_use]
    fn draw_vertices(&self, count: u32, offset: u32) -> Result<(), Box<Error>> {
        // let frame = &self.swapchain_frames[self.frame_index as usize];

        // unsafe {
        //     self.device.cmd_draw(frame.command_buffer, count, 1, offset, 0);

        //     self.device.cmd_end_render_pass(frame.command_buffer);
        //     self.device.end_command_buffer(frame.command_buffer)?;
        // };

        Ok(())
    }

    #[must_use]
    fn begin_frame(&mut self) -> Result<(), Box<Error>> {
        unsafe { self.device.queue_wait_idle(self.graphics_queue)? };

        let frame = &self.swapchain_frames[self.frame_index as usize];

        let index = unsafe {
            self.swapchain_loader.acquire_next_image_khr(
                self.swapchain,
                u64::MAX,
                frame.acquire_image_semaphore,
                vk::Fence::null(),
            )?
        };

        self.frame_index = index;

        Ok(())
    }

    #[must_use]
    fn end_frame(&mut self) -> Result<(), Box<Error>> {
        println!("{}", self.frame_index);

        let frame = &self.swapchain_frames[self.frame_index as usize];

        let submit_info = vk::SubmitInfo {
            s_type: vk::StructureType::SubmitInfo,
            p_next: ptr::null(),
            wait_semaphore_count: 1,
            p_wait_semaphores: [frame.acquire_image_semaphore].as_ptr(),
            p_wait_dst_stage_mask: [].as_ptr(),
            command_buffer_count: 1,
            p_command_buffers: &frame.command_buffer,
            signal_semaphore_count: 1,
            p_signal_semaphores: [frame.render_finished_semaphore].as_ptr(),
        };

        unsafe {
            self.device
                .queue_submit(self.graphics_queue, &[submit_info], vk::Fence::null())?;

            let present_info = vk::PresentInfoKHR {
                s_type: vk::StructureType::PresentInfoKhr,
                p_next: ptr::null(),
                wait_semaphore_count: 1,
                p_wait_semaphores: [frame.render_finished_semaphore].as_ptr(),
                swapchain_count: 1,
                p_swapchains: &self.swapchain,
                p_image_indices: &self.frame_index,
                p_results: ptr::null_mut(),
            };

            self.swapchain_loader
                .queue_present_khr(self.graphics_queue, &present_info)?;
        }

        Ok(())
    }

    #[must_use]
    fn update_resolution(&self, width: u64, height: u64) -> Result<(), Box<Error>> {
        Ok(())
    }

    #[must_use]
    fn change_settings(&self) -> Result<(), Box<Error>> {
        Ok(())
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
    Err(
        "Unable to find suitable memory index for depth image.".to_owned(),
    )
}
