use ash::vk;
use ash::version::DeviceV1_0;

use std::ptr;
use std::error::Error;

use super::DeviceV10;
use super::SurfaceDetails;

pub fn create_framebuffers(
    device: &DeviceV10,
    present_image_views: Vec<vk::ImageView>,
    depth_image_view: vk::ImageView,
    render_pass: vk::RenderPass,
    surface: &SurfaceDetails,
) -> Result<Vec<vk::Framebuffer>, Box<Error>> {
    present_image_views
        .iter()
        .map(|&present_image_view| {
            let framebuffer_attachments = [present_image_view, depth_image_view];
            let frame_buffer_create_info = vk::FramebufferCreateInfo {
                s_type: vk::StructureType::FramebufferCreateInfo,
                p_next: ptr::null(),
                flags: Default::default(),
                render_pass: render_pass,
                attachment_count: framebuffer_attachments.len() as u32,
                p_attachments: framebuffer_attachments.as_ptr(),
                width: surface.resolution.width,
                height: surface.resolution.height,
                layers: 1,
            };
            device.create_framebuffer(&frame_buffer_create_info, None)
        })
        .collect()?
}