use crate::prelude::*;
use sdl2::{gfx::primitives::DrawRenderer, render::Canvas, video::Window};

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

pub struct Keyboard<'a> {
    config: &'a Config,
    keyboard_config: KeyboardConfig,
}

impl Keyboard<'_> {
    pub fn new(config: &Config) -> Result<Keyboard> {
        Ok(Keyboard {
            config,
            keyboard_config: KeyboardConfig::load(&config.current_keyboard_layout)?,
        })
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<()> {
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
