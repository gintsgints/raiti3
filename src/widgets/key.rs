use sdl2::{
    gfx::primitives::DrawRenderer, pixels::Color, rect::Rect, render::Canvas, ttf::Sdl2TtfContext,
    video::Window,
};

use crate::prelude::*;

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

    if !keyspec.label1.is_empty() {
        text_box(
            canvas,
            ttf_context,
            &FontSpec {
                font_name: "MesloLGS NF Regular.ttf",
                point_size: 12,
            },
            &mut Rect::new(
                point.0 as i32 + 5,
                point.1 as i32 + 5,
                (width - 10) as u32,
                (height / 2) as u32,
            ),
            &keyspec.label1,
            &keyspec.label1_align,
        )?;
    }
    if !keyspec.label2.is_empty() {
        text_box(
            canvas,
            ttf_context,
            &FontSpec {
                font_name: "MesloLGS NF Regular.ttf",
                point_size: 12,
            },
            &mut Rect::new(
                point.0 as i32 + 5,
                (point.1 + height / 2) as i32,
                (width - 10) as u32,
                (height / 2) as u32,
            ),
            &keyspec.label2,
            &keyspec.label2_align,
        )?;
    }
    Ok(width)
}
