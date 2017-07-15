use std::path::Path;
use std::fs;
use glium::{Frame, SwapBuffersError};
use libloading::Library;
use world::World;


pub struct GameLib(Library);

impl GameLib {
    pub fn new(lib_path: &str) -> GameLib {
        let lib_copy_path = Path::new(lib_path).with_extension("module");
        let _ = fs::copy(lib_path, lib_copy_path.to_owned());
        GameLib(Library::new(lib_copy_path).unwrap())
    }

    pub fn render(&self, frame: Frame) -> Result<(), SwapBuffersError> {
        unsafe {
            let func = self.0
                .get::<fn(Frame) -> Result<(), SwapBuffersError>>(b"render")
                .unwrap();
            func(frame)
        }
    }

    pub fn update(&self, world: &World, next_world: &mut World) {
        unsafe {
            let func = self.0.get::<fn(&World, &mut World)>(b"update").unwrap();
            func(world, next_world)
        }
    }
}
