extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate libloading;

pub mod code_reload;


use gfx::Device;
use gfx::format::{Rgba8, DepthStencil};

use code_reload::Game;

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

    let (window, mut device, mut factory, _main_color, mut _main_depth) =
        gfx_window_glutin::init::<Rgba8, DepthStencil>(builder);

    let mut game = Game::new(LIB_PATH);
    let mut last_modified = std::fs::metadata(LIB_PATH)
        .unwrap()
        .modified()
        .unwrap();

    let mut encoder = factory.create_command_buffer().into();

    'main: loop {
        if let Ok(Ok(modified)) = std::fs::metadata(LIB_PATH).map(|m| m.modified()) {
            if modified > last_modified {
                drop(game);
                game = Game::new(LIB_PATH);
                last_modified = modified;
            }
        }

        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                _ => {}
            }
        }

        game.render_and_update(&mut encoder);

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
