use crate::constants::{FONT, WINDOW_WIDTH};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf;
use sdl2::video::Window;

pub const SCOREBOARD_PADDING: i32 = 28;
pub const SCOREBOARD_FONT_SIZE: i32 = 48;

pub struct Scoreboard {
    pub left_score: i32,
    pub right_score: i32,
}

impl Scoreboard {
    fn draw_score(&self, score: &str, x: i32, y: i32, canvas: &mut Canvas<Window>) {
        let ttf = ttf::init().unwrap();
        let mut font = ttf.load_font(FONT, SCOREBOARD_FONT_SIZE as u16).unwrap();
        font.set_style(ttf::FontStyle::NORMAL);
        let surface = font.render(score).blended(Color::WHITE).unwrap();
        let (w, h) = font.size_of(&score).unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .unwrap();
        canvas
            .copy(
                &texture,
                None,
                Rect::new((x as u32 - w / 2).try_into().unwrap(), y, w, h),
            )
            .unwrap();
    }
    pub fn draw_scoreboard(&self, canvas: &mut Canvas<Window>) {
        self.draw_score(
            &self.left_score.to_string(),
            WINDOW_WIDTH / 4,
            SCOREBOARD_PADDING,
            canvas,
        );

        self.draw_score(
            &self.right_score.to_string(),
            (WINDOW_WIDTH / 4) * 3,
            SCOREBOARD_PADDING,
            canvas,
        );
    }
}
