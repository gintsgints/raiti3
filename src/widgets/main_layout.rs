use sdl2::{EventPump, rect::Rect, render::Canvas, video::Window};

use crate::{
    Keyboard,
    prelude::*,
    widgets::{self},
};

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

pub fn main_layout(
    canvas: &mut Canvas<Window>,
    keyboard: &mut Keyboard,
    header_font: &sdl2::ttf::Font<'_, 'static>,
    description_font: &sdl2::ttf::Font<'_, 'static>,
    key_font: &sdl2::ttf::Font<'_, 'static>,
    events: &EventPump,
) -> Result<()> {
    let section_height = canvas.viewport().size().1 / 4;

    // Top description section
    let top_rect = Rect::new(20, 20, canvas.viewport().size().0 - 40, section_height - 40);
    let mut header_rect = Rect::new(top_rect.x, top_rect.y, top_rect.width(), 50);
    let mut description_rect = Rect::new(
        top_rect.x,
        top_rect.y + 50,
        top_rect.width(),
        section_height - 50,
    );

    // Middle section for keyboard
    let middle_rect = Rect::new(
        20,
        section_height as i32,
        canvas.viewport().size().0 - 40,
        section_height * 2,
    );

    // Bottom section for additional information and exercises
    let mut bottom_rect = Rect::new(
        20,
        section_height as i32 * 3,
        canvas.viewport().size().0 - 40,
        section_height,
    );

    widgets::text_box(
        canvas,
        header_font,
        &mut header_rect,
        "Overview",
        &Align::Center,
    )?;
    widgets::text_box(
        canvas,
        description_font,
        &mut description_rect,
        "This is a first lesson of keyboard touch typing. In this lesson we will provide information which will help you with teaching course.",
        &Align::Left,
    )?;

    keyboard.draw(events, canvas, key_font, middle_rect)?;

    widgets::text_box(
        canvas,
        description_font,
        &mut bottom_rect,
        "Press <Enter> to continue",
        &Align::Left,
    )?;

    Ok(())
}
