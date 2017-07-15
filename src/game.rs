use glium::{Frame, SwapBuffersError, Surface};
use world::World;
use system;

#[no_mangle]
pub fn render(frame: &mut Frame, world: &World) -> Result<(), SwapBuffersError> {
    //frame.clear_color(0.0, 0.0, 1.0, 1.0);
    frame.set_finish()
}

#[no_mangle]
pub fn update(world: &World, next_world: &mut World) {
    system::process_physics(world, next_world)
}

#[no_mangle]
pub fn interpolate(world: &World, next_world: &mut World, alpha: f64) {

}