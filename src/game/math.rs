pub type Vec3 = Vec4;

#[derive(Debug, PartialEq)]
pub struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec4 {
        Vec4 {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        }
    }

    pub fn zero() -> Vec4 {
        Vec4::new(0.0, 0.0, 0.0)
    }

    pub fn add(&mut self, other: &Vec4) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
        self.w = self.w + other.w;
    }
}
