use sdl2::{event::Event, pixels::Color, rect::Rect, render::TextureQuery};
use std::time::Duration;

mod config;
mod keyboard_config;
mod prelude;
mod widgets;

use crate::config::assets_dir;
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

        let section_height = canvas.viewport().size().1 / 3;

        let top_rect = Rect::new(20, 20, canvas.viewport().size().0 - 40, section_height - 40);
        let middle_rect = Rect::new(
            20,
            section_height as i32,
            canvas.viewport().size().0 - 40,
            section_height,
        );
        // let bottom_rect = Rect::new(0, section_height as i32 * 2, canvas.viewport().size().0, section_height);

        let normal_font = ttf_context.load_font(assets_dir()?.join("MesloLGS NF Regular.ttf"), 18)?;
        let texture_creator = canvas.texture_creator();
        let surface = normal_font
        .render("This is a first lesson of keyboard touch typing. In this lesson we will provide ")
        .blended_wrapped(Color::BLACK, top_rect.width())?;
        let texture = texture_creator.create_texture_from_surface(&surface)?;
        let TextureQuery { width, height, .. } = texture.query();

        let target = Rect::new(top_rect.x, top_rect.y, width, height);
        canvas.copy(&texture, None, Some(target))?;

        keyboard.draw(&events, &mut canvas, middle_rect)?;
        canvas.present();

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
