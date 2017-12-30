use ash::vk;
use ash::version::DeviceV1_0;

use std::ptr;
use std::error::Error;

pub fn new(
    device: &DeviceV1_0,
    surface_format: &vk::SurfaceFormatKHR,
) -> Result<vk::RenderPass, Box<Error>> {
    let renderpass_attachments = [
        vk::AttachmentDescription {
            format: surface_format.format,
            flags: vk::AttachmentDescriptionFlags::empty(),
            samples: vk::SAMPLE_COUNT_1_BIT,
            load_op: vk::AttachmentLoadOp::Clear,
            store_op: vk::AttachmentStoreOp::Store,
            stencil_load_op: vk::AttachmentLoadOp::DontCare,
            stencil_store_op: vk::AttachmentStoreOp::DontCare,
            initial_layout: vk::ImageLayout::Undefined,
            final_layout: vk::ImageLayout::PresentSrcKhr,
        },
        vk::AttachmentDescription {
            format: vk::Format::D16Unorm,
            flags: vk::AttachmentDescriptionFlags::empty(),
            samples: vk::SAMPLE_COUNT_1_BIT,
            load_op: vk::AttachmentLoadOp::Clear,
            store_op: vk::AttachmentStoreOp::DontCare,
            stencil_load_op: vk::AttachmentLoadOp::DontCare,
            stencil_store_op: vk::AttachmentStoreOp::DontCare,
            initial_layout: vk::ImageLayout::DepthStencilAttachmentOptimal,
            final_layout: vk::ImageLayout::DepthStencilAttachmentOptimal,
        },
    ];

    let dependency = vk::SubpassDependency {
        dependency_flags: Default::default(),
        src_subpass: vk::VK_SUBPASS_EXTERNAL,
        dst_subpass: Default::default(),
        src_stage_mask: vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
        src_access_mask: Default::default(),
        dst_access_mask: vk::ACCESS_COLOR_ATTACHMENT_READ_BIT |
            vk::ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
        dst_stage_mask: vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
    };

    let subpass = {
        let color_attachment_ref = vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::ColorAttachmentOptimal,
        };
        let depth_attachment_ref = vk::AttachmentReference {
            attachment: 1,
            layout: vk::ImageLayout::DepthStencilAttachmentOptimal,
        };

        vk::SubpassDescription {
            color_attachment_count: 1,
            p_color_attachments: &color_attachment_ref,
            p_depth_stencil_attachment: &depth_attachment_ref,
            flags: Default::default(),
            pipeline_bind_point: vk::PipelineBindPoint::Graphics,
            input_attachment_count: 0,
            p_input_attachments: ptr::null(),
            p_resolve_attachments: ptr::null(),
            preserve_attachment_count: 0,
            p_preserve_attachments: ptr::null(),
        }
    };

    let renderpass_create_info = vk::RenderPassCreateInfo {
        s_type: vk::StructureType::RenderPassCreateInfo,
        flags: Default::default(),
        p_next: ptr::null(),
        attachment_count: renderpass_attachments.len() as u32,
        p_attachments: renderpass_attachments.as_ptr(),
        subpass_count: 1,
        p_subpasses: &subpass,
        dependency_count: 1,
        p_dependencies: &dependency,
    };

    let render_pass = unsafe { device.create_render_pass(&renderpass_create_info, None)? };

    Ok(render_pass)
}