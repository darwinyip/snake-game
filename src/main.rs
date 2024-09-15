extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use game_context::{GameContext, GridSize};
use renderer::Renderer;

mod game_context;
mod renderer;

const GRID_X_SIZE: i32 = 40;
const GRID_Y_SIZE: i32 = 30;
const DOT_SIZE_IN_PXS: i32 = 20;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Snake Game",
            (GRID_X_SIZE * DOT_SIZE_IN_PXS).try_into().unwrap(),
            (GRID_Y_SIZE * DOT_SIZE_IN_PXS).try_into().unwrap()
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = Renderer::new(window, DOT_SIZE_IN_PXS)?;

    let mut event_pump = sdl_context.event_pump()?;

    let grid_size = GridSize{ width: GRID_X_SIZE, height: GRID_Y_SIZE};
    let mut context = GameContext::new(grid_size);

    renderer.draw(&context)?;

    let mut frame_counter = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::W => context.move_up(),
                        Keycode::A => context.move_left(),
                        Keycode::S => context.move_down(),
                        Keycode::D => context.move_right(),
                        Keycode::Escape => context.toggle_pause(),
                        Keycode::Return => context = GameContext::new(grid_size),
                        _ => {}
                    }
                }
                 _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        frame_counter += 1;
        if frame_counter % 10 == 0 {
            context.next_tick();
            frame_counter = 0;
        }

        renderer.draw(&context)?;
    }

    Ok(())
}
