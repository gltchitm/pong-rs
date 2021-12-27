use crate::constants::{FONT, WINDOW_WIDTH};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf;
use sdl2::video::Window;

pub const WELCOME_TOP_FONT_SIZE: i32 = 64;
pub const WELCOME_BOTTOM_FONT_SIZE: i32 = 24;
pub const WELCOME_TOP_Y_POS: i32 = WINDOW_WIDTH / 3;
pub const WELCOME_BOTTOM_Y_POS: i32 = WINDOW_WIDTH / 2;
pub const WELCOME_TOP_TEXT: &str = "Pong RS";
pub const WELCOME_BOTTOM_TEXT: &str = "Press SPACE To Start!";
pub const WELCOME_TEXT_COLOR: Color = Color::RGB(195, 195, 195);

fn draw_text(message: &str, y: i32, font_size: i32, color: Color, canvas: &mut Canvas<Window>) {
    let ttf = ttf::init().unwrap();
    let mut font = ttf.load_font(FONT, font_size as u16).unwrap();
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
            Rect::new(WINDOW_WIDTH / 2 - w as i32 / 2, y, w, h),
        )
        .unwrap();
}

pub fn draw_welcome(canvas: &mut Canvas<Window>) {
    draw_text(
        WELCOME_TOP_TEXT,
        WELCOME_TOP_Y_POS,
        WELCOME_TOP_FONT_SIZE,
        WELCOME_TEXT_COLOR,
        canvas,
    );
    draw_text(
        WELCOME_BOTTOM_TEXT,
        WELCOME_BOTTOM_Y_POS,
        WELCOME_BOTTOM_FONT_SIZE,
        WELCOME_TEXT_COLOR,
        canvas,
    );
}
