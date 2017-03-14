#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate libloading;

use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::time;
use std::time::SystemTime;

#[cfg(unix)]
use libloading::os::unix::{Library, Symbol};
#[cfg(windows)]
use libloading::os::windows::{Library, Symbol};

use gfx::Device;
use gfx::format::{Rgba8, DepthStencil};

fn has_been_modified(lib_path: &Path, last_modified_time: SystemTime) -> bool {
    if let Ok(metadata) = lib_path.metadata() {
        match metadata.modified() {
            Ok(time) => time > last_modified_time,
            Err(_) => false,
        }
    } else {
        false
    }
}

fn lib_last_modified(lib_path: &Path) -> SystemTime {
    lib_path.metadata()
        .and_then(|m| m.modified())
        .unwrap()
}

fn copy_game_lib(lib_path: &Path) -> Result<u64, io::Error> {
    let copy_path = lib_path.with_extension("module");
    fs::copy(lib_path, copy_path)
}

pub fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("Xtreme Game".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();

    let (window, mut device, mut _factory, _main_color, mut _main_depth) =
        gfx_window_glutin::init::<Rgba8, DepthStencil>(builder);

    let mut game_running = true;

    let game_lib = if cfg!(target_os = "windows") {
        Path::new("./target/debug/xtreme_game.dll")
    } else if cfg!(target_os = "macos") {
        Path::new("./target/debug/libxtreme_game.dylib")
    } else {
        Path::new("./target/debug/libxtreme_game.so")
    };

    let game_module = game_lib.with_extension("module");
    let mut game_code: Library;
    let mut hello_func: Symbol<fn() -> String>;
    let mut last_modified = time::UNIX_EPOCH;

    while game_running {
        if has_been_modified(game_lib, last_modified) {
            drop(game_code);
            drop(hello_func);

            let a = copy_game_lib(game_lib);
            println!("{:?}", a);
            game_code = Library::new(game_module.as_path()).unwrap();
            hello_func = unsafe { game_code.get(b"hello").unwrap() };
            last_modified = lib_last_modified(game_lib);

            println!("{}", hello_func());
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
