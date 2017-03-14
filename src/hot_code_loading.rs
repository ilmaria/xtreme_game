extern crate libloading;

use std::path::Path;
use std::fs;

use self::libloading::Library;

use super::game::math::Vec3;


pub struct GameLib(Library);

impl GameLib {
    pub fn new(lib_path: &str) -> GameLib {
        let lib_copy_path = Path::new(lib_path).with_extension("module");
        let _ = fs::copy(lib_path, lib_copy_path.to_owned());
        GameLib(Library::new(lib_copy_path).unwrap())
    }

    pub fn hello(&self) -> String {
        unsafe {
            let func = self.0.get::<fn() -> String>(b"hello").unwrap();
            func()
        }
    }

    pub fn vec_test(&self) -> Vec3 {
        unsafe {
            let func = self.0.get::<fn() -> Vec3>(b"vec_test").unwrap();
            func()
        }
    }
}
