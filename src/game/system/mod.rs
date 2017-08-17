use std::error::Error;

use super::state::State;
use super::component;
use super::asset;
use super::super::renderer::Renderer;

pub fn process_physics(state: &State, next_state: &mut State) {
    for (i, obj) in state.physics_components.iter().enumerate() {
        next_state.physics_components[i] =
            integrate(obj, state.delta_time.subsec_nanos() as f32 / 1_000_000.0);
    }
}

fn integrate(component: &Option<component::Physics>, dt: f32) -> Option<component::Physics> {
    if let &Some(ref obj) = component {
        let mut result = component::Physics::new();
        result.momentum += obj.calculate_forces() * dt;
        result.pos += obj.momentum * obj.inv_mass * dt;
        Some(result)
    } else {
        None
    }
}

pub fn draw_entities(renderer: &mut Renderer, state: &mut State) -> Result<(), Box<Error>> {
    for (id, graphics_component) in state.graphics_components.iter_mut().enumerate() {
        if let &mut Some(ref mut component) = graphics_component {
            if component.mesh.loading_state == asset::LoadingState::Unloaded {
                component.mesh.load()?;
                renderer.update_model(id as u32, &component.mesh.vertices)?;
            }

            if component.mesh.descriptors_changed {
                renderer
                    .update_descriptors(id as u32, &component.mesh.vertices)?;
            }
        }
    }

    Ok(())
}
