use super::constants::{BALL_MOVEMENT, WINDOW_HEIGHT, WINDOW_WIDTH};

use rand::prelude::random;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Ball {
    pub x: i32,
    pub y: i32,
    pub x_movement: i32,
    pub y_movement: i32
}

impl Ball {
    pub const RADIUS: i32 = 13;
    pub const X_CENTER: i32 = WINDOW_WIDTH / 2 - Ball::RADIUS / 2;
    pub const Y_CENTER: i32 = WINDOW_HEIGHT / 2 - Ball::RADIUS / 2;

    pub fn draw_ball(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::WHITE);
        for w in 0..(Ball::RADIUS * 2) {
            for h in 0..(Ball::RADIUS * 2) {
                let dx = Ball::RADIUS - w;
                let dy = Ball::RADIUS - h;
                if dx * dx + dy * dy <= Ball::RADIUS * Ball::RADIUS {
                    canvas
                        .draw_point(Point::new(dx + self.x, dy + self.y))
                        .unwrap();
                }
            }
        }
        canvas.set_draw_color(Color::BLACK);
    }
    pub fn reset(&mut self) {
        self.x = Ball::X_CENTER;
        self.y = Ball::Y_CENTER;
        self.x_movement = Ball::rand_movement();
        self.y_movement = Ball::rand_movement();
    }
    pub fn rand_movement() -> i32 {
        if random() {
            BALL_MOVEMENT
        } else {
            -BALL_MOVEMENT
        }
    }
}
