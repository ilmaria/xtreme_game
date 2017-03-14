#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate libloading;

use gfx::Device;
use gfx::format::{Rgba8, DepthStencil};

mod hot_code_loading;
use hot_code_loading::GameLib;

#[cfg(windows)]
const LIB_PATH: &'static str = "./target/debug/xtreme_game.dll";
#[cfg(linux)]
const LIB_PATH: &'static str = "./target/debug/libxtreme_game.so";
#[cfg(macos)]
const LIB_PATH: &'static str = "./target/debug/libxtreme_game.dylib";


pub fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("Xtreme Game".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();

    let (window, mut device, mut _factory, _main_color, mut _main_depth) =
        gfx_window_glutin::init::<Rgba8, DepthStencil>(builder);

    let mut game_running = true;

    let mut game_lib = GameLib::new(LIB_PATH);
    let mut last_modified = std::fs::metadata(LIB_PATH)
        .unwrap()
        .modified()
        .unwrap();

    while game_running {
        if let Ok(Ok(modified)) = std::fs::metadata(LIB_PATH).map(|m| m.modified()) {
            if modified > last_modified {
                drop(game_lib);
                game_lib = GameLib::new(LIB_PATH);
                last_modified = modified;

                println!("{}", game_lib.hello());
            }
        }

        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => game_running = false,
                _ => {}
            }
        }

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
