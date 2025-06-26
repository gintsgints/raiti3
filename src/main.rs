use sdl2::event::Event;
use std::time::Duration;

mod config;
mod keyboard_config;
mod prelude;
mod widgets;
mod common;

pub use crate::prelude::*;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

fn main() -> Result<()> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init()?;

    let config = Config::load()?;
    let mut keyboard = Keyboard::new(&config)?;

    let window = video_subsystem
        .window("Raiti", 800, 600)
        .position_centered()
        .resizable()
        .build()?;

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

        main_layout(&mut canvas, &mut keyboard, &ttf_context, &events)?;

        canvas.present();

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
