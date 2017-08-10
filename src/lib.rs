#[macro_use]
extern crate ash;
extern crate cgmath;
extern crate glsl_to_spirv;

pub mod game;
pub mod renderer;

use std::error::Error;

use game::state::State;
use game::system;
use renderer::Renderer;

#[no_mangle]
pub fn render(renderer: &Renderer, state: &mut State) -> Result<(), Box<Error>> {
    system::draw_entities(renderer, state)
}

#[no_mangle]
pub fn update(state: &State, next_state: &mut State) {
    system::process_physics(state, next_state);
}

#[no_mangle]
pub fn interpolate(state: &State, next_state: &mut State, alpha: f64) {}
