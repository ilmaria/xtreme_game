use cgmath::Vector3;

#[derive(Debug)]
pub struct Physics {
    pub pos: Vector3<f32>,
    pub momentum: f32,
    pub inv_mass: f32,
}

// impl Physics {
//     fn recalc_vel(&mut self) {
//         self.vel = self.momentum * self.inv_mass;
//     }
// }

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