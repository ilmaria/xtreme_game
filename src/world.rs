use cgmath::Vector3;
use std::time;

#[derive(Debug)]
struct World {
    time: time::Instant,
    physics_components: Components<Physics>,
    graphics_components: Components<Graphics>,
    sound_components: Components<Sound>,
    AI_components: Components<AI>,
    entities: Components<Entity>,
}

struct Components<T> {
    hot: Vec<T>,
    cold: Vec<T>,
}

struct Physics {
    pos: Vector3<f32>,
}

struct Graphics {
    pos: Vector3<f32>,
}

struct Sound {
    pos: Vector3<f32>,
}

struct AIComponent {
    pos: Vector3<f32>,
}

struct Entity {
    pos: Vector3<f32>,
}