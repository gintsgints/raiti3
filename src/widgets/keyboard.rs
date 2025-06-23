use std::collections::HashSet;

use crate::prelude::*;
use sdl2::{gfx::primitives::DrawRenderer, keyboard::Keycode, render::Canvas, video::Window, EventPump};

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

pub struct Keyboard {
    prev_keys: HashSet<Keycode>,
    keyboard_config: KeyboardConfig,
}

impl Keyboard {
    pub fn new(config: &Config) -> Result<Keyboard> {
        Ok(Keyboard {
            keyboard_config: KeyboardConfig::load(&config.current_keyboard_layout)?,
            prev_keys: HashSet::new(),
        })
    }

    pub fn draw(&mut self, events: &EventPump, canvas: &mut Canvas<Window>) -> Result<()> {
        // Create a set of pressed Keys.
        let keys = events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        // Get the difference between the new and old sets.
        let new_keys = &keys - &self.prev_keys;
        let old_keys = &self.prev_keys - &keys;

        if !new_keys.is_empty() || !old_keys.is_empty() {
            println!("new_keys: {:?}\told_keys:{:?}", new_keys, old_keys);
        }

        self.prev_keys = keys;

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.rounded_box(
            100,
            100,
            300,
            200,
            self.keyboard_config.keyboard_corner_curve,
            sdl2::pixels::Color::RGB(0, 0, 255),
        )?;
        Ok(())
    }
}
