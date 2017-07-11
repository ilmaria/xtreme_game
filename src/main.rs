extern crate glium;
extern crate libloading;

pub mod code_reload;

use glium::DisplayBuild;

use code_reload::GameLib;

#[cfg(target_os = "windows")]
const LIB_PATH: &str = "./target/debug/xtreme_game.dll";
#[cfg(target_os = "linux")]
const LIB_PATH: &str = "./target/debug/libxtreme_game.so";
#[cfg(target_os = "macos")]
const LIB_PATH: &str = "./target/debug/libxtreme_game.dylib";


pub fn main() {
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Xtreme Game".to_string())
        .with_dimensions(1024, 768)
        .with_gl_robustness(glium::glutin::Robustness::RobustLoseContextOnReset)
        .with_vsync()
        .build_glium()
        .unwrap();

    let mut game = GameLib::new(LIB_PATH);
    let mut last_modified = std::fs::metadata(LIB_PATH).unwrap().modified().unwrap();

    'main: loop {
        if let Ok(Ok(modified)) = std::fs::metadata(LIB_PATH).map(|m| m.modified()) {
            if modified > last_modified {
                drop(game);
                game = GameLib::new(LIB_PATH);
                last_modified = modified;
            }
        }

        for event in window.poll_events() {
            match event {
                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                glium::glutin::Event::Closed => break 'main,
                _ => {}
            }
        }

        let frame = window.draw();
        game.render(frame).unwrap();
    }
}
