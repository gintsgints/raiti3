use crate::{keyboard_config::KeySpec, widgets::text_box};
use sdl2::{
    gfx::primitives::DrawRenderer, pixels::Color, rect::Rect, render::Canvas, ttf::Sdl2TtfContext,
    video::Window,
};

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

pub fn key(
    canvas: &mut Canvas<Window>,
    ttf_context: &Sdl2TtfContext,
    point: (i16, i16),
    height: i16,
    keyboard_corner_curve: i16,
    key_color: Color,
    keyspec: &KeySpec,
) -> Result<i16> {
    let width: i16 = (height as f32 * keyspec.width_ratio) as i16;

    canvas.rounded_box(
        point.0,
        point.1,
        point.0 + width,
        point.1 + height,
        keyboard_corner_curve,
        key_color,
    )?;

    text_box(
        canvas,
        ttf_context,
        Rect::new(
            point.0 as i32 + 5,
            point.1 as i32 + 5,
            (point.0 + width) as u32,
            ((point.1 + height) / 2) as u32,
        ),
        12,
        &keyspec.label1,
    )?;
    if !keyspec.label2.is_empty() {
        text_box(
            canvas,
            ttf_context,
            Rect::new(
                point.0 as i32 + 5,
                (point.1 + height / 2) as i32,
                (point.0 + width) as u32,
                ((point.1 + height) / 2) as u32,
            ),
            12,
            &keyspec.label2,
        )?;
    }
    Ok(width)
}
