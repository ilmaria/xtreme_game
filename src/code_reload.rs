use std::path::Path;
use std::fs;

use libloading::Library;
use gfx;


pub struct Game(Library);

impl Game {
    pub fn new(lib_path: &str) -> Game {
        let lib_copy_path = Path::new(lib_path).with_extension("module");
        let _ = fs::copy(lib_path, lib_copy_path.to_owned());
        Game(Library::new(lib_copy_path).unwrap())
    }

    pub fn render_and_update<R, C, F>(&self, encoder: &mut gfx::Encoder<R, C>, factory: &mut F)
        where R: gfx::Resources,
              C: gfx::CommandBuffer<R>,
              F: gfx::Factory<R>
    {
        unsafe {
            let func = self.0
                .get::<fn(&mut gfx::Encoder<R, C>, &mut F)>(b"render_and_update_gl")
                .unwrap();
            func(encoder, factory)
        }
    }
}
