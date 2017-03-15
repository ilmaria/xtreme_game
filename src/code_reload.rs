use std::path::Path;
use std::fs;

use libloading::Library;
use glutin::Window;


pub struct GameLib(Library);

impl GameLib {
    pub fn new(lib_path: &str) -> GameLib {
        let lib_copy_path = Path::new(lib_path).with_extension("module");
        let _ = fs::copy(lib_path, lib_copy_path.to_owned());
        GameLib(Library::new(lib_copy_path).unwrap())
    }

    pub fn render_and_update(&self, window: &Window) {
        unsafe {
            let func = self.0.get::<fn(&Window)>(b"render_and_update").unwrap();
            func(window)
        }
    }
}
