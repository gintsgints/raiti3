use sdl2::{event::Event, rect::Rect};
use std::time::Duration;

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
    let mut keyboard = Keyboard::new(&config)?;

    let window = video_subsystem
        .window("Raiti", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build()?;

    let mut events = sdl_context.event_pump()?;

    'running: loop {
        for event in events.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            };
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0xFF, 0xFF, 0xFF));
        canvas.clear();

        let section_height = canvas.viewport().size().1 / 3;

        // let top_rect = Rect::new(0, 0, canvas.viewport().size().0,  section_height);
        let middle_rect = Rect::new(20, section_height as i32, canvas.viewport().size().0 - 40, section_height);
        // let bottom_rect = Rect::new(0, section_height as i32 * 2, canvas.viewport().size().0, section_height);

        keyboard.draw(&events, &mut canvas, middle_rect)?;
        canvas.present();

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
