#[macro_use]
extern crate ash;
extern crate cgmath;

pub mod game;

use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::swapchain;
use vulkano::sync::GpuFuture;

use std::error::Error;

use game::state::State;
use game::system;
use game::vulkan::RenderParams;

#[no_mangle]
pub fn render(render_params: RenderParams, state: &State) -> Result<GpuFuture, Box<Error>> {
    let (image_num, acquire_future) = swapchain::acquire_next_image(render_params.swapchain.clone(),
                                                                        None)?;

    let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(render_params.device, render_params.queue.family())?
        .begin_render_pass(render_params.framebuffers[image_num].clone(), false,
                            vec![[0.0, 0.0, 1.0, 1.0].into()])?
        .draw(render_params.pipeline, DynamicState::none(), render_params.vertex_buffer, (), ())?
        .end_render_pass()?
        .build()?;

    render_params.previous_frame.join(acquire_future)
        .then_execute(render_params.queue, command_buffer)?
        .then_swapchain_present(render_params.queue, render_params.swapchain, image_num)
        .then_signal_fence_and_flush()?
}

#[no_mangle]
pub fn update(state: &State, next_state: &mut State) {
    system::process_physics(state, next_state)
}

#[no_mangle]
pub fn interpolate(state: &State, next_state: &mut State, alpha: f64) {

}