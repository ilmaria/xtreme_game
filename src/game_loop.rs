// use gfx::{Device, Factory, RenderTarget, DepthStencil};
// use gfx::format::{RenderFormat, DepthFormat};
use glutin::Window;

use math::Vec3;

#[no_mangle]
pub fn render_and_update(window: &Window) {}

#[no_mangle]
pub fn vec_test() -> Vec3 {
    let mut a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(5.0, 6.0, 7.0);
    let mut d = vec![1.2, 3.6, 1.0, 8.6];

    for i in 0..1000 {
        let c = vec![22.2, i as f32, 1.0, 8.7];
        d[0] += c[0];
        d[1] += c[1];
        d[2] += c[2];
        d[3] += c[3];
    }
    if a.x == 1.0 { a } else { b }
}

#[cfg(test)]
mod tests {
    use super::{vec_test, Vec3};

    #[test]
    fn it_works() {}
}
