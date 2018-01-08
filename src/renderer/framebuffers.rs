use ash::vk;
use ash::version::DeviceV1_0;

use std::ptr;
use std::error::Error;

use super::VkDevice;
use super::surface::Surface;

pub fn new(
    device: &VkDevice,
    render_pass: vk::RenderPass,
    surface: Surface,
    present_image_views: &Vec<vk::ImageView>,
    depth_image_view: vk::ImageView,
) -> Result<Vec<vk::Framebuffer>, Box<Error>> {
    let framebuffers = present_image_views
        .iter()
        .map(|&present_image_view| {
            let attachments = [present_image_view, depth_image_view];
            let create_info = vk::FramebufferCreateInfo {
                s_type: vk::StructureType::FramebufferCreateInfo,
                p_next: ptr::null(),
                flags: Default::default(),
                render_pass: render_pass,
                attachment_count: attachments.len() as u32,
                p_attachments: attachments.as_ptr(),
                width: surface_resolution.width,
                height: surface_resolution.height,
                layers: 1,
            };
            unsafe { device.create_framebuffer(&create_info, None).unwrap() }
        })
        .collect();

    Ok(framebuffers)
}
