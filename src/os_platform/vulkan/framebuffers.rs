use ash::vk;
use ash::version::DeviceV1_0;

use std::ptr;
use std::error::Error;

use super::Renderer;
use super::RendererError;

impl Renderer {
    pub fn create_framebuffers(&mut self) -> Result<&mut Renderer, Box<Error>> {
        let device = self.device.ok_or(RendererError::NoDevice)?;
        let render_pass = self.render_pass.ok_or(RendererError::NoRenderPass)?;
        let surface_resolution = self.surface_resolution.ok_or(
            RendererError::NoSurfaceResolution,
        )?;
        let present_image_views = self.present_image_views.ok_or(
            RendererError::NoPresentImageViews,
        )?;
        let depth_image_view = self.depth_image_view.ok_or(RendererError::NoDepthImageView)?;

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
                device.create_framebuffer(&frame_buffer_create_info, None)
            })
            .collect()?;

        self.framebuffers = Some(framebuffers);

        Ok(self)
    }
}