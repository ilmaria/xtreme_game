pub mod graphics;
pub mod physics;

use cgmath::Vector3;

pub use self::graphics::Graphics;
pub use self::physics::Physics;

#[derive(Debug, Clone)]
pub struct Sound {
    pub pos: Vector3<f32>,
}

#[derive(Debug, Clone)]
pub struct AI {
    pub pos: Vector3<f32>,
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub pos: Vector3<f32>,
}
