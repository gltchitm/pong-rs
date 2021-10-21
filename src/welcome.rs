use super::constants;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf;
use sdl2::video::Window;

fn draw_text(message: &str, y: i32, font_size: i32, color: Color, canvas: &mut Canvas<Window>) {
    let ttf = ttf::init().unwrap();
    let mut font = ttf.load_font(constants::FONT, font_size as u16).unwrap();
    font.set_style(ttf::FontStyle::NORMAL);
    let surface = font.render(message).blended(color).unwrap();
    let (w, h) = font.size_of(message).unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(surface)
        .unwrap();
    canvas
        .copy(
            &texture,
            None,
            Rect::new(constants::WINDOW_WIDTH / 2 - w as i32 / 2, y, w, h)
        )
        .unwrap();
}

pub fn draw_welcome(canvas: &mut Canvas<Window>) {
    draw_text(
        constants::WELCOME_TOP_TEXT,
        constants::WELCOME_TOP_Y_POS,
        constants::WELCOME_TOP_FONT_SIZE,
        constants::WELCOME_TEXT_COLOR,
        canvas
    );
    draw_text(
        constants::WELCOME_BOTTOM_TEXT,
        constants::WELCOME_BOTTOM_Y_POS,
        constants::WELCOME_BOTTOM_FONT_SIZE,
        constants::WELCOME_TEXT_COLOR,
        canvas
    );
}
