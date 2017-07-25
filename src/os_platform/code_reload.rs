use libloading::Library;

use std::path::Path;
use std::fs;
use std::error::Error;

use game::state::State;

pub struct GameLib(Library);

impl GameLib {
    pub fn new(lib_path: &str) -> GameLib {
        let lib_copy_path = Path::new(lib_path).with_extension("module");
        let _ = fs::copy(lib_path, lib_copy_path.to_owned());
        GameLib(Library::new(lib_copy_path).unwrap())
    }

    pub fn render(&self, state: &State) -> Result<String, Box<Error>> {
        unsafe {
            let func = self.0
                .get::<fn(&State) -> Result<String, Box<Error>>>(b"render")
                .unwrap();
            func(state)
        }
    }

    pub fn update(&self, state: &State, next_state: &mut State) {
        unsafe {
            let func = self.0.get::<fn(&State, &mut State)>(b"update").unwrap();
            func(state, next_state)
        }
    }

    pub fn interpolate(&self, state: &State, next_state: &mut State, alpha: f64) {
        unsafe {
            let func = self.0
                .get::<fn(&State, &mut State, alpha: f64)>(b"interpolate")
                .unwrap();
            func(state, next_state, alpha)
        }
    }
}
