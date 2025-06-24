use std::collections::HashSet;

use crate::{prelude::*, widgets::text_box};
use sdl2::{
    EventPump, gfx::primitives::DrawRenderer, keyboard::Keycode, pixels::Color, rect::Rect,
    render::Canvas, ttf::Sdl2TtfContext, video::Window,
};

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

pub struct Keyboard {
    keyboard_config: KeyboardConfig,
    default_key_color: Color,
    pressed_key_color: Color,
    #[cfg(debug_assertions)]
    prev_keys: HashSet<i32>,
}

impl Keyboard {
    pub fn new(config: &Config) -> Result<Keyboard> {
        Ok(Keyboard {
            keyboard_config: KeyboardConfig::load(&config.current_keyboard_layout)?,
            default_key_color: Color::RGB(0xD1, 0xD1, 0xD1),
            pressed_key_color: Color::RGB(0x91, 0x91, 0x91),
            #[cfg(debug_assertions)]
            prev_keys: HashSet::new(),
        })
    }

    pub fn draw(
        &mut self,
        events: &EventPump,
        canvas: &mut Canvas<Window>,
        ttf_context: &Sdl2TtfContext,
        area: Rect,
    ) -> Result<()> {
        // Create a set of pressed Keys.
        let keys: HashSet<i32> = events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .map(|keycode| keycode.into_i32())
            .collect();

        #[cfg(debug_assertions)]
        {
            // Get the difference between the new and old sets.
            let new_keys = &keys - &self.prev_keys;
            let old_keys = &self.prev_keys - &keys;

            if !new_keys.is_empty() || !old_keys.is_empty() {
                println!("new_keys: {:?}\told_keys:{:?}", new_keys, old_keys);
            }
        }

        // TODO: Find out why keyboard is not drawn in the middle of the screen.
        let simple_key_width = (area.size().0 as i16
            - 2 * self.keyboard_config.keyboard_side_padding)
            / self.keyboard_config.cols_for_keys;

        let mut key_y = area.y as i16 + self.keyboard_config.keyboard_side_padding;
        for row in self.keyboard_config.rows.iter() {
            let mut key_x = area.x as i16 + self.keyboard_config.keyboard_side_padding;
            for keyspec in row.keys.iter() {
                let width: i16 = (simple_key_width as f32 * keyspec.width_ratio) as i16;
                // Check if the key is pressed
                let key_color = if keys.contains(&keyspec.key_code) {
                    self.pressed_key_color
                } else {
                    self.default_key_color
                };
                canvas.rounded_box(
                    key_x,
                    key_y,
                    key_x + width,
                    key_y + simple_key_width,
                    self.keyboard_config.keyboard_corner_curve,
                    key_color,
                )?;

                text_box(
                    canvas,
                    ttf_context,
                    Rect::new(
                        key_x as i32 + 5,
                        key_y as i32 + 5,
                        (key_x + width) as u32,
                        ((key_y + simple_key_width) / 2) as u32,
                    ),
                    12,
                    &keyspec.label1,
                )?;
                if !keyspec.label2.is_empty() {
                    text_box(
                        canvas,
                        ttf_context,
                        Rect::new(
                            key_x as i32 + 5,
                            (key_y + simple_key_width / 2) as i32,
                            (key_x + width) as u32,
                            ((key_y + simple_key_width) / 2) as u32,
                        ),
                        12,
                        &keyspec.label2,
                    )?;
                }

                key_x += self.keyboard_config.keyboard_side_padding + width;
            }
            key_y += simple_key_width + self.keyboard_config.space_between_keys;
        }

        #[cfg(debug_assertions)]
        {
            self.prev_keys = keys;
        }

        Ok(())
    }
}
