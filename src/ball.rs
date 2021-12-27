use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

use rand::prelude::random;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Ball {
    pub x: i32,
    pub y: i32,
    pub x_velocity: i32,
    pub y_velocity: i32,
}

impl Ball {
    pub const RADIUS: i32 = 13;
    pub const VELOCITY: i32 = 22;
    pub const ARROW_SIZE: i32 = 40;
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
    pub fn draw_arrow(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::WHITE);

        let center = WINDOW_WIDTH / 2 - Ball::RADIUS / 2;

        if self.x_velocity > 0 && self.y_velocity < 0 {
            canvas
                .draw_line(
                    Point::new(center + Ball::RADIUS, center - Ball::RADIUS),
                    Point::new(
                        center + Ball::RADIUS + Ball::ARROW_SIZE,
                        center - Ball::RADIUS - Ball::ARROW_SIZE,
                    ),
                )
                .unwrap();
            canvas
                .draw_line(
                    Point::new(
                        center + Ball::RADIUS,
                        center - Ball::RADIUS - Ball::ARROW_SIZE,
                    ),
                    Point::new(
                        center + Ball::RADIUS + Ball::ARROW_SIZE,
                        center - Ball::RADIUS - Ball::ARROW_SIZE,
                    ),
                )
                .unwrap();
            canvas
                .draw_line(
                    Point::new(
                        center + Ball::RADIUS + Ball::ARROW_SIZE,
                        center - Ball::RADIUS,
                    ),
                    Point::new(
                        center + Ball::RADIUS + Ball::ARROW_SIZE,
                        center - Ball::RADIUS - Ball::ARROW_SIZE,
                    ),
                )
                .unwrap();
        }

        if self.x_velocity > 0 && self.y_velocity > 0 {
            canvas
                .draw_line(
                    Point::new(center + Ball::RADIUS, center + Ball::RADIUS),
                    Point::new(
                        center + Ball::RADIUS + Ball::ARROW_SIZE,
                        center + Ball::RADIUS + Ball::ARROW_SIZE,
                    ),
                )
                .unwrap();
            canvas
                .draw_line(
                    Point::new(
                        center + Ball::RADIUS,
                        center + Ball::RADIUS + Ball::ARROW_SIZE,
                    ),
                    Point::new(
                        center + Ball::RADIUS + Ball::ARROW_SIZE,
                        center + Ball::RADIUS + Ball::ARROW_SIZE,
                    ),
                )
                .unwrap();
            canvas
                .draw_line(
                    Point::new(
                        center + Ball::RADIUS + Ball::ARROW_SIZE,
                        center + Ball::RADIUS,
                    ),
                    Point::new(
                        center + Ball::RADIUS + Ball::ARROW_SIZE,
                        center + Ball::RADIUS + Ball::ARROW_SIZE,
                    ),
                )
                .unwrap();
        }

        if self.x_velocity < 0 && self.y_velocity > 0 {
            canvas
                .draw_line(
                    Point::new(center - Ball::RADIUS, center + Ball::RADIUS),
                    Point::new(
                        center - Ball::RADIUS - Ball::ARROW_SIZE,
                        center + Ball::RADIUS + Ball::ARROW_SIZE,
                    ),
                )
                .unwrap();
            canvas
                .draw_line(
                    Point::new(
                        center - Ball::RADIUS,
                        center + Ball::RADIUS + Ball::ARROW_SIZE,
                    ),
                    Point::new(
                        center - Ball::RADIUS - Ball::ARROW_SIZE,
                        center + Ball::RADIUS + Ball::ARROW_SIZE,
                    ),
                )
                .unwrap();
            canvas
                .draw_line(
                    Point::new(
                        center - Ball::RADIUS - Ball::ARROW_SIZE,
                        center + Ball::RADIUS,
                    ),
                    Point::new(
                        center - Ball::RADIUS - Ball::ARROW_SIZE,
                        center + Ball::RADIUS + Ball::ARROW_SIZE,
                    ),
                )
                .unwrap();
        }

        if self.x_velocity < 0 && self.y_velocity < 0 {
            canvas
                .draw_line(
                    Point::new(center - Ball::RADIUS, center - Ball::RADIUS),
                    Point::new(
                        center - Ball::RADIUS - Ball::ARROW_SIZE,
                        center - Ball::RADIUS - Ball::ARROW_SIZE,
                    ),
                )
                .unwrap();
            canvas
                .draw_line(
                    Point::new(
                        center - Ball::RADIUS,
                        center - Ball::RADIUS - Ball::ARROW_SIZE,
                    ),
                    Point::new(
                        center - Ball::RADIUS - Ball::ARROW_SIZE,
                        center - Ball::RADIUS - Ball::ARROW_SIZE,
                    ),
                )
                .unwrap();
            canvas
                .draw_line(
                    Point::new(
                        center - Ball::RADIUS - Ball::ARROW_SIZE,
                        center - Ball::RADIUS,
                    ),
                    Point::new(
                        center - Ball::RADIUS - Ball::ARROW_SIZE,
                        center - Ball::RADIUS - Ball::ARROW_SIZE,
                    ),
                )
                .unwrap();
        }
    }
    pub fn reset(&mut self) {
        self.x = Ball::X_CENTER;
        self.y = Ball::Y_CENTER;
        self.x_velocity = Ball::rand_velocity();
        self.y_velocity = Ball::rand_velocity();
    }
    pub fn rand_velocity() -> i32 {
        if random() {
            Ball::VELOCITY
        } else {
            -Ball::VELOCITY
        }
    }
}
