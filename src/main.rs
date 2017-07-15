extern crate glium;
extern crate libloading;
extern crate cgmath;

pub mod code_reload;
pub mod world;
pub mod component;

use glium::DisplayBuild;
use world::World;
use code_reload::GameLib;
use std::mem;
use std::time;

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

    let mut world = World {
        delta_time: time::Duration::from_millis(16),
        physics_components: Vec::with_capacity(128),
        graphics_components: Vec::with_capacity(128),
        sound_components: Vec::with_capacity(128),
        ai_components: Vec::with_capacity(128),
        entities: Vec::with_capacity(128),
    };

    let mut next_world = World {
        delta_time: time::Duration::from_millis(16),
        physics_components: Vec::with_capacity(128),
        graphics_components: Vec::with_capacity(128),
        sound_components: Vec::with_capacity(128),
        ai_components: Vec::with_capacity(128),
        entities: Vec::with_capacity(128),
    };

    let mut curr_time = time::Instant::now();
    let mut time_accumulator = time::Duration::new(0, 0);

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

        let new_time = time::Instant::now();
        let mut frame_time = new_time - curr_time;

        if frame_time > time::Duration::from_millis(250) {
            frame_time = time::Duration::from_millis(250);
        }

        curr_time = new_time;
        time_accumulator += frame_time;

        while time_accumulator >= world.delta_time {
            mem::swap(&mut next_world, &mut world);
            game.update(&world, &mut next_world);
            time_accumulator -= world.delta_time;
        }

        let alpha = time_accumulator.subsec_nanos() as f64 / world.delta_time.subsec_nanos() as f64;

        game.interpolate(&world, &mut next_world, alpha);

        let frame = window.draw();
        game.render(frame, &next_world).unwrap();
    }
}
