use ash::vk;
use ash::version::DeviceV1_0;

use std::ptr;
use std::error::Error;

use super::VulkanRenderer;
use super::RendererError;

impl VulkanRenderer {
    pub fn create_framebuffers(
        device: &DeviceV1_0,
        render_pass: vk::RenderPass,
        surface_resolution: vk::Extent2D,
        present_image_views: Vec<vk::ImageView>,
        depth_image_view: vk::ImageView,
    ) -> Result<Vec<vk::Framebuffer>, Box<Error>> {
        let framebuffers = present_image_views
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
                    width: surface_resolution.width,
                    height: surface_resolution.height,
                    layers: 1,
                };
                unsafe {
                    device
                        .create_framebuffer(&frame_buffer_create_info, None)
                        .unwrap()
                }
            })
            .collect();

        Ok(framebuffers)
    }
}