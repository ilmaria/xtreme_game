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

    pub fn render(&self, frame: Frame, world: &World) -> Result<(), SwapBuffersError> {
        unsafe {
            let func = self.0
                .get::<fn(Frame, &World) -> Result<(), SwapBuffersError>>(b"render")
                .unwrap();
            func(frame, world)
        }
    }

    pub fn update(&self, world: &World, next_world: &mut World) {
        unsafe {
            let func = self.0.get::<fn(&World, &mut World)>(b"update").unwrap();
            func(world, next_world)
        }
    }

    pub fn interpolate(&self, world: &World, next_world: &mut World, alpha: f64) {
        unsafe {
            let func = self.0
                .get::<fn(&World, &mut World, alpha: f64)>(b"interpolate")
                .unwrap();
            func(world, next_world, alpha)
        }
    }
}
