use sdl2::{EventPump, rect::Rect, render::Canvas, ttf::Sdl2TtfContext, video::Window};

use crate::{
    Keyboard,
    widgets::{self},
};

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

pub fn main_layout(
    canvas: &mut Canvas<Window>,
    keyboard: &mut Keyboard,
    ttf_context: &Sdl2TtfContext,
    events: &EventPump,
) -> Result<()> {
    let section_height = canvas.viewport().size().1 / 3;

    let top_rect = Rect::new(20, 20, canvas.viewport().size().0 - 40, section_height - 40);
    let header_rect = Rect::new(top_rect.x, top_rect.y, top_rect.width(), 50);
    let description_rect = Rect::new(top_rect.x, top_rect.y + 50, top_rect.width(), section_height - 50);

    let middle_rect = Rect::new(
        20,
        section_height as i32,
        canvas.viewport().size().0 - 40,
        section_height,
    );
    // let bottom_rect = Rect::new(0, section_height as i32 * 2, canvas.viewport().size().0, section_height);

    widgets::text_box(canvas, ttf_context, header_rect, 24, "Overview")?;
    widgets::text_box(canvas, ttf_context, description_rect, 16, "This is a first lesson of keyboard touch typing. In this lesson we will provide ")?;

    keyboard.draw(events, canvas, ttf_context, middle_rect)?;
    Ok(())
}
