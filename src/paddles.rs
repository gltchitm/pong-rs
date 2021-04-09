use super::constants::{ WINDOW_WIDTH, WINDOW_HEIGHT, PADDING };
use super::ball::Ball;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

pub struct Paddles {
    pub left: i32,
    pub right: i32,
    pub left_movement: i32,
    pub right_movement: i32
}

impl Paddles {
    const PADDLE_WIDTH: i32 = 15;
    const PADDLE_HEIGHT: i32 = 200;
    pub const TOP: i32 = PADDING;
    pub const CENTER: i32 = WINDOW_WIDTH / 2 - Paddles::PADDLE_HEIGHT / 2;
    pub const BOTTOM: i32 = WINDOW_HEIGHT - Paddles::PADDLE_HEIGHT - PADDING;
    
    fn draw_paddle(x: i32, y: i32, canvas: &mut Canvas<Window>) {
        let rect = Rect::new(
            x,
            y,
            Paddles::PADDLE_WIDTH as u32,
            Paddles::PADDLE_HEIGHT as u32
        );
        canvas.fill_rect(rect).unwrap();
    }
    pub fn draw_paddles(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::WHITE);
        Paddles::draw_paddle(PADDING, self.left, canvas);
        Paddles::draw_paddle(
            WINDOW_WIDTH - Paddles::PADDLE_WIDTH - PADDING,
            self.right,
            canvas
        );
        canvas.set_draw_color(Color::BLACK);
    }
    pub fn ball_does_collide_with_left_paddle(&self, ball: &Ball) -> bool {
        PADDING + Paddles::PADDLE_WIDTH < ball.x + Ball::RADIUS &&
        PADDING + Paddles::PADDLE_WIDTH * 2 > ball.x &&
        self.left < ball.y + Ball::RADIUS &&
        self.left + Paddles::PADDLE_HEIGHT > ball.y
    }
    pub fn ball_does_collide_with_right_paddle(&self, ball: &Ball) -> bool {
        WINDOW_WIDTH - (PADDING + Paddles::PADDLE_WIDTH) < ball.x + Ball::RADIUS &&
        (WINDOW_WIDTH - (PADDING + Paddles::PADDLE_WIDTH)) * 2 > ball.x &&
        self.right < ball.y + Ball::RADIUS &&
        self.right + Paddles::PADDLE_HEIGHT > ball.y
    }
}