extern crate libloading;
extern crate cgmath;
extern crate ash;
extern crate winit;

pub mod os_platform;
pub mod game;

#[macro_use]
use ash::vk;
use winit::{VirtualKeycode, Event, WindowEvent};

use std::mem;
use std::cmp;
use std::time;

use game::state::State;
use game::vulkan::RenderParams;
use os_platform::code_reload::GameLib;

#[cfg(target_os = "windows")]
const LIB_PATH: &str = "./target/debug/xtreme_game.dll";
#[cfg(target_os = "linux")]
const LIB_PATH: &str = "./target/debug/libxtreme_game.so";
#[cfg(target_os = "macos")]
const LIB_PATH: &str = "./target/debug/libxtreme_game.dylib";


pub fn main() {
    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new()
        .with_title("Xtreme Game")
        .with_dimensions(1024, 768)
        .build()
        .unwrap();

    os_platform::init_vulkan();

    let mut game = GameLib::new(LIB_PATH);
    let mut last_modified = std::fs::metadata(LIB_PATH).unwrap().modified().unwrap();

    let mut state = State::default();
    let mut next_state = State::default();

    let mut curr_time = time::Instant::now();
    let mut time_accumulator = time::Duration::new(0, 0);

    let mut previous_frame = Box::new(now(device.clone())) as Box<GpuFuture>;

    'main: loop {
        if let Ok(Ok(modified)) = std::fs::metadata(LIB_PATH).map(|m| m.modified()) {
            if modified > last_modified {
                drop(game);
                game = GameLib::new(LIB_PATH);
                last_modified = modified;
            }
        }

        events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput {
                    input: KeyboardInput { virtual_keycode: Some(VirtualKeycode::Escape), .. }, ..
                },
                ..
            } |
            Event::WindowEvent { event: WindowEvent::Closed, .. } => break 'main,
            _ => (),
        });

        let new_time = time::Instant::now();
        let frame_time = cmp::min(new_time - curr_time, time::Duration::from_millis(250));
        curr_time = new_time;
        time_accumulator += frame_time;

        while time_accumulator >= state.delta_time {
            mem::swap(&mut next_state, &mut state);
            game.update(&state, &mut next_state);
            time_accumulator -= state.delta_time;
        }

        let alpha = time_accumulator.subsec_nanos() as f64 / state.delta_time.subsec_nanos() as f64;
        game.interpolate(&state, &mut next_state, alpha);

        previous_frame.cleanup_finished();

        let future = game.render(RenderParams {
            device.clone(),
            queue.clone(),
            swapchain.clone(),
            render_pass.clone(),
            pipeline.clone(),
            &framebuffers,
            &vertex_buffer,
            previous_frame,
        }, &next_state).unwrap();

        previous_frame = Box::new(future) as Box<_>;
    }
}
