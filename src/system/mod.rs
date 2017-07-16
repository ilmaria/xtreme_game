use world::World;
use component;

pub fn process_physics(world: &World, next_world: &mut World) {
    for (i, obj) in world.physics_components.iter().enumerate() {
        next_world.physics_components[i] =
            integrate(obj, world.delta_time.subsec_nanos() as f32 / 1_000_000.0);
    }
}

fn integrate(obj: &component::Physics, dt: f32) -> component::Physics {
    let mut result = component::Physics::new();
    result.momentum += obj.calculate_forces() * dt;
    result.pos += obj.momentum * obj.inv_mass * dt;
    result
}