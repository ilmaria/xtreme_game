use vulkano as vk;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::device::{Device, Queue};
use vulkano::framebuffer::{Framebuffer, Subpass, RenderPass};
use vulkano::instance::Instance;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::shader::ShaderModule;
use vulkano::swapchain;
use vulkano::swapchain::{PresentMode, SurfaceTransform, Swapchain};
use vulkano::sync::now;
use vulkano::sync::GpuFuture;
use vulkano_win::VkSurfaceBuild;
use vulkano_shaders;
use vulkano_shaders::parse;
use glsl_to_spirv;
use winit::{VirtualKeycode, Event, WindowEvent};

use std::path;
use std::error::Error;
use std::sync::Arc;

use game::vulkan::Vertex;
use game::vulkan::RenderParams;

pub fn init_vulkan()
    -> Result<
    (Instance,
     Device,
     Queue,
     Swapchain,
     RenderPass,
     GraphicsPipeline,
     Vec<Arc<Framebuffer>>,
     Arc<CpuAccessibleBuffer<Vertex>>),
    Box<Error>,
>
{
    let instance = {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None)?
    };

    // Let's take first available physical device.
    let physical = vk::instance::PhysicalDevice::enumerate(&instance).next()?;
    // Some little debug infos.
    println!(
        "Using device: {} (type: {:?})",
        physical.name(),
        physical.ty()
    );

    let queue = physical.queue_families().find(|&q| {
        // We take the first queue that supports drawing to our window.
        q.supports_graphics() && window.surface().is_supported(q).unwrap_or(false)
    })?;

    // Logical device initialization.
    let (device, mut queues) = {
        let device_ext = vk::device::DeviceExtensions {
            khr_swapchain: true,
            ..vk::device::DeviceExtensions::none()
        };

        Device::new(
            physical,
            physical.supported_features(),
            &device_ext,
            [(queue, 0.5)].iter().cloned(),
        )?
    };

    let queue = queues.next()?;

    let (swapchain, images) = {
        let caps = window.surface().capabilities(physical)?;

        let dimensions = caps.current_extent.unwrap_or([1280, 1024]);
        let alpha = caps.supported_composite_alpha.iter().next()?;
        let format = caps.supported_formats[0].0;

        Swapchain::new(
            device.clone(),
            window.surface().clone(),
            caps.min_image_count,
            format,
            dimensions,
            1,
            caps.supported_usage_flags,
            &queue,
            SurfaceTransform::Identity,
            alpha,
            PresentMode::Fifo,
            true,
            None,
        )?
    };

    let vertex_buffer = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::all(),
        Some(queue.family()),
        [
            Vertex { position: [-0.5, -0.25] },
            Vertex { position: [0.0, 0.5] },
            Vertex { position: [0.25, -0.1] },
        ].iter()
            .cloned(),
    )?;

    let fs = create_shader_module(
        device.clone(),
        "../../data/shaders/cube.frag",
        glsl_to_spirv::ShaderType::Fragment,
    )?;
    let vs = create_shader_module(
        device.clone(),
        "../../data/shaders/cube.vert",
        glsl_to_spirv::ShaderType::Vertex,
    )?;

    let render_pass = Arc::new(single_pass_renderpass!(device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swapchain.format(),
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }
    )?);

    let pipeline = Arc::new(GraphicsPipeline::start()
        .vertex_input_single_buffer()
        .vertex_shader(vs.main_entry_point(), ())
        .triangle_list()
        .viewports(iter::once(Viewport {
            origin: [0.0, 0.0],
            depth_range: 0.0..1.0,
            dimensions: [
                images[0].dimensions()[0] as f32,
                images[0].dimensions()[1] as f32,
            ],
        }))
        .fragment_shader(fs.main_entry_point(), ())
        .render_pass(Subpass::from(render_pass.clone(), 0)?)
        .build(device.clone())?);

    let framebuffers = images
        .iter()
        .map(|image| {
            Arc::new(Framebuffer::start(render_pass.clone())
                .add(image.clone())?
                .build()?)
        })
        .collect::<Vec<_>>();

    Ok((
        instance,
        device,
        queue,
        swapchain,
        render_pass,
        pipeline,
        framebuffers,
        vertex_buffer,
    ))
}

fn create_shader_module(
    device: Device,
    path: &str,
    shader_type: glsl_to_spirv::ShaderType,
) -> Result<Arc<ShaderModule<Device>>, Box<Error>> {
    let spirv = {
        let shader = include_bytes!(path);
        let spirv_code = glsl_to_spirv::compile(shader, shader_type)?;
        let mut spirv_data = Vec::new();
        spirv_code.read_to_end(&mut spirv_data)?;
        spirv_data
    };

    unsafe {
        ShaderModule::new(device, spirv);
    }
}
