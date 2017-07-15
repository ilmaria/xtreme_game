use world::World;

pub fn process_physics(world: &World, next_world: &mut World) {
    next_world.delta_time = world.delta_time + 1.0;
}