use cgmath::Vector3;

#[derive(Debug, Clone)]
pub struct Physics {
    pub pos: Vector3<f32>,
    pub momentum: Vector3<f32>,
    pub inv_mass: f32,
}

impl Physics {
    pub fn new() -> Physics {
        Physics {
            pos: Vector3::new(0.0, 0.0, 0.0),
            momentum: Vector3::new(0.0, 0.0, 0.0),
            inv_mass: 1.0,
        }
    }

    pub fn calculate_forces(&self) -> Vector3<f32> {
        Vector3::new(0.0, -9.81, 0.0)
    }
}
