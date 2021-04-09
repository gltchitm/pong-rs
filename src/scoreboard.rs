use super::constants::{ WINDOW_WIDTH, SCOREBOARD_PADDING, SCOREBOARD_FONT_SIZE, FONT };

use sdl2::ttf;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

pub struct Scoreboard {
    pub left_score: i32,
    pub right_score: i32
}

impl Scoreboard {
    pub fn draw_scoreboard(&self, canvas: &mut Canvas<Window>) {
        let score_str = format!("{}     {}", self.left_score, self.right_score);
        let ttf = ttf::init().unwrap();
        let mut font = ttf.load_font(
            FONT,
            SCOREBOARD_FONT_SIZE as u16
        ).unwrap();
        font.set_style(ttf::FontStyle::NORMAL);
        let surface = font.render(&score_str).blended(Color::WHITE).unwrap();
        let (w, h) = font.size_of(&score_str).unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(
            surface
        ).unwrap();
        canvas.copy(&texture, None, Rect::new(
            (WINDOW_WIDTH - w as i32) / 4 * 2,
            SCOREBOARD_PADDING,
            w,
            h
        )).unwrap();
    }
}