use sdl2::{pixels::Color, rect::Rect, render::{Canvas, TextureQuery}, ttf::Sdl2TtfContext, video::Window};

use crate::config::assets_dir;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

pub fn text_box(canvas: &mut Canvas<Window>, ttf_context: &Sdl2TtfContext, area: Rect, point_size: u16, text: &str) -> Result<()> {
        let normal_font = ttf_context.load_font(assets_dir()?.join("MesloLGS NF Regular.ttf"), point_size)?;
        let texture_creator = canvas.texture_creator();
        let surface = normal_font
        .render(text)
        .blended_wrapped(Color::BLACK, area.width())?;
        let texture = texture_creator.create_texture_from_surface(&surface)?;
        let TextureQuery { width, height, .. } = texture.query();

        let target = Rect::new(area.x, area.y, width, height);
        canvas.copy(&texture, None, Some(target))?;
        Ok(())
}
