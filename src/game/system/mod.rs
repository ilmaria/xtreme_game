use std::error::Error;

use super::state::State;
use super::component;
use super::super::renderer::Renderer;

pub fn process_physics(state: &State, next_state: &mut State) {
    for (i, obj) in state.physics_components.iter().enumerate() {
        next_state.physics_components[i] =
            integrate(obj, state.delta_time.subsec_nanos() as f32 / 1_000_000.0);
    }
}

fn integrate(obj: &component::Physics, dt: f32) -> component::Physics {
    let mut result = component::Physics::new();
    result.momentum += obj.calculate_forces() * dt;
    result.pos += obj.momentum * obj.inv_mass * dt;
    result
}

pub fn draw_entities(renderer: &Renderer, state: &mut State) -> Result<(), Box<Error>> {
    for graphics_component in state.graphics_components.iter_mut() {
        let model = &mut graphics_component.model3d;
        model.load(renderer)?;

        renderer
            .draw_vertices(model.offset, model.vertices.len() as u32)?;
    }

    Ok(())
}
