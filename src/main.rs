use sdl2::{event::Event, keyboard::Keycode};
use std::{collections::HashSet, time::Duration};

mod config;
mod keyboard_config;
mod prelude;
mod widgets;

pub use crate::prelude::*;
pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

fn main() -> Result<()> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let config = Config::load()?;
    let keyboard = Keyboard::new(&config)?;

    let window = video_subsystem
        .window("Raiti", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build()?;
    keyboard.draw(&mut canvas)?;
    canvas.present();

    let mut events = sdl_context.event_pump()?;

    let mut prev_keys = HashSet::new();

    'running: loop {
        for event in events.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            };
        }

        // Create a set of pressed Keys.
        let keys = events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        // Get the difference between the new and old sets.
        let new_keys = &keys - &prev_keys;
        let old_keys = &prev_keys - &keys;

        if !new_keys.is_empty() || !old_keys.is_empty() {
            println!("new_keys: {:?}\told_keys:{:?}", new_keys, old_keys);
        }

        prev_keys = keys;

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
