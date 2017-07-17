use vulkano::device::Device;
use vulkano::device::Queue;
use vulkano::framebuffer::Framebuffer;
use vulkano::framebuffer::RenderPass;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::swapchain::Swapchain;
use vulkano::buffer::cpu_access::CpuAccessibleBuffer;
use vulkano::sync::GpuFuture;

use std::sync::Arc;

#[derive(Debug, Clone)]
struct Vertex {
    position: [f32; 2],
}
impl_vertex!(Vertex, position);

pub struct RenderParams {
    device: Device,
    queue: Queue,
    swapchain: Swapchain,
    render_pass: RenderPass,
    pipeline: GraphicsPipeline,
    framebuffers: Vec<Arc<Framebuffer>>,
    vertex_buffer: Arc<CpuAccessibleBuffer<Vertex>>,
    previous_frame: Box<GpuFuture>,
}