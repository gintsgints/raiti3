use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureQuery},
    ttf::Sdl2TtfContext,
    video::Window,
};

use crate::prelude::*;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

pub struct FontSpec<'a> {
    pub font_name: &'a str,
    pub point_size: u16,
}

pub fn text_box(
    canvas: &mut Canvas<Window>,
    ttf_context: &Sdl2TtfContext,
    font_spec: &FontSpec,
    area: &mut Rect,
    text: &str,
    align: &Align,
) -> Result<()> {
    let normal_font: sdl2::ttf::Font<'_, 'static> =
        ttf_context.load_font(assets_dir()?.join(font_spec.font_name), font_spec.point_size)?;
    let texture_creator = canvas.texture_creator();
    let surface = normal_font
        .render(text)
        .blended_wrapped(Color::BLACK, area.width())?;
    let texture = texture_creator.create_texture_from_surface(&surface)?;
    let TextureQuery { width, height, .. } = texture.query();

    match align {
        Align::Left => {
            // No adjustment needed for left alignment
        }
        Align::Center => {
            // Center the text horizontally
            let centered_x = area.x() + ((area.width() as i32 - width as i32) / 2);
            area.set_x(centered_x);
        }
        Align::Right => {
            // Align to the right
            let right_x = area.x() + area.width() as i32 - width as i32;
            area.set_x(right_x);
        }
    }

    let target = Rect::new(area.x, area.y, width, height);

    canvas.copy(&texture, None, Some(target))?;
    Ok(())
}
