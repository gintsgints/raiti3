use sdl2::event::Event;
use std::time::Duration;

mod common;
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
    let ttf_context = sdl2::ttf::init()?;
    let header_font: sdl2::ttf::Font<'_, 'static> =
        ttf_context.load_font(assets_dir()?.join("OpenSans-Light.ttf"), 24)?;
    let description_font: sdl2::ttf::Font<'_, 'static> =
        ttf_context.load_font(assets_dir()?.join("OpenSans-Light.ttf"), 16)?;
    let key_font: sdl2::ttf::Font<'_, 'static> =
        ttf_context.load_font(assets_dir()?.join("MesloLGS NF Regular.ttf"), 12)?;

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

        main_layout(&mut canvas, &mut keyboard, &header_font, &description_font,  &key_font, &events)?;

        canvas.present();

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
