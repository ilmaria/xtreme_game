use glium::{Frame, SwapBuffersError, Surface};

#[no_mangle]
pub fn render(frame: &mut Frame) -> Result<(), SwapBuffersError> {
    //frame.clear_color(0.0, 0.0, 1.0, 1.0);
    frame.set_finish()
}