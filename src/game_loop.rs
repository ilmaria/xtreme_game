// use gfx::{Device, Factory, RenderTarget, DepthStencil};
// use gfx::format::{RenderFormat, DepthFormat};
use gfx;
use gfx_device_gl as gfx_gl;

use math::Vec3;


#[no_mangle]
pub fn render_and_update_gl(encoder: &mut gfx::Encoder<gfx_gl::Resources, gfx_gl::CommandBuffer>,
                            factory: &mut gfx_gl::Factory) {
}
