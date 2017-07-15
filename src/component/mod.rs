use cgmath::Vector3;

#[derive(Debug)]
pub struct Physics {
    pub pos: Vector3<f32>,
}

#[derive(Debug)]
pub struct Graphics {
    pub pos: Vector3<f32>,
}

#[derive(Debug)]
pub struct Sound {
    pub pos: Vector3<f32>,
}

#[derive(Debug)]
pub struct AI {
    pub pos: Vector3<f32>,
}

#[derive(Debug)]
pub struct Entity {
    pub pos: Vector3<f32>,
}