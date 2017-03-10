#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate libloading as lib;

use std::path::PathBuf;
use std::ffi;

use gfx::Device;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

fn game_lib(file: &str) -> PathBuf {
    let mut lib_file = PathBuf::from(file);

    if cfg!(target_os = "windows") {
        lib_file.set_extension("dll");
    } else if cfg!(target_os = "macos") {
        lib_file.set_extension("dylib");
    } else {
        lib_file.set_extension("so");
    }

    lib_file
}

pub fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("Xtreme Game".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();

    let (window, mut device, mut _factory, _main_color, mut _main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let mut game_running = true;

    let game_lib = game_lib("./target/debug/game_lib");
    let game_lib_name = match game_lib.file_name() {
        Some(name) => name,
        None => &ffi::OsStr::new(""),
    };

    let game_code = lib::Library::new(game_lib_name).unwrap();
    let func: lib::Symbol<fn() -> String> = unsafe { game_code.get(b"hello").unwrap() };

    println!("{}", func());

    while game_running {
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
