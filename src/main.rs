mod ball;
mod constants;
mod paddles;
mod pressed_keys;
mod scoreboard;
mod welcome;

use ball::Ball;
use constants::{
    PADDING, TICKS_UNTIL_START, TICK_DURATION, WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH,
};
use paddles::Paddles;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn default_paddles() -> Paddles {
    Paddles {
        left: Paddles::CENTER,
        right: Paddles::CENTER,
        left_velocity: 0,
        right_velocity: 0,
    }
}

fn default_ball() -> Ball {
    Ball {
        x: Ball::X_CENTER,
        y: Ball::Y_CENTER,
        x_velocity: Ball::rand_velocity(),
        y_velocity: Ball::rand_velocity(),
    }
}

fn default_scoreboard() -> scoreboard::Scoreboard {
    scoreboard::Scoreboard {
        left_score: 0,
        right_score: 0,
    }
}

fn default_pressed_keys() -> pressed_keys::PressedKeys {
    pressed_keys::PressedKeys {
        w: false,
        s: false,
        up: false,
        down: false,
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut paddles = default_paddles();
    let mut ball = default_ball();
    let mut scoreboard = default_scoreboard();
    let mut pressed_keys = default_pressed_keys();

    let mut show_welcome = true;

    let mut tick: u128 = 0;

    'pong: loop {
        let tick_instant = Instant::now();

        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'pong,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    show_welcome = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    if !show_welcome {
                        pressed_keys.w = true;
                        paddles.left_velocity = -Paddles::VELOCITY;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    if !show_welcome {
                        pressed_keys.s = true;
                        paddles.left_velocity = Paddles::VELOCITY;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if !show_welcome {
                        pressed_keys.up = true;
                        paddles.right_velocity = -Paddles::VELOCITY;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if !show_welcome {
                        pressed_keys.down = true;
                        paddles.right_velocity = Paddles::VELOCITY;
                    }
                }
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => pressed_keys.w = false,
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => pressed_keys.s = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => pressed_keys.up = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => pressed_keys.down = false,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    paddles = default_paddles();
                    ball = default_ball();
                    scoreboard = default_scoreboard();
                    pressed_keys = default_pressed_keys();
                    tick = 0;
                    show_welcome = true;
                }
                _ => {}
            }
        }

        if show_welcome {
            welcome::draw_welcome(&mut canvas);
        } else {
            if !pressed_keys.w && !pressed_keys.s {
                paddles.left_velocity = 0;
            } else if pressed_keys.w && !pressed_keys.s {
                paddles.left_velocity = -Paddles::VELOCITY;
            } else if pressed_keys.s && !pressed_keys.w {
                paddles.left_velocity = Paddles::VELOCITY;
            }

            if !pressed_keys.up && !pressed_keys.down {
                paddles.right_velocity = 0;
            } else if pressed_keys.up && !pressed_keys.down {
                paddles.right_velocity = -Paddles::VELOCITY;
            } else if pressed_keys.down && !pressed_keys.up {
                paddles.right_velocity = Paddles::VELOCITY;
            }

            if !show_welcome {
                tick += 1;

                paddles.left = (paddles.left + paddles.left_velocity)
                    .clamp(PADDING, WINDOW_HEIGHT - Paddles::PADDLE_HEIGHT - PADDING);
                paddles.right = (paddles.right + paddles.right_velocity)
                    .clamp(PADDING, WINDOW_HEIGHT - Paddles::PADDLE_HEIGHT - PADDING);

                let ticks_until_start_sixth = TICKS_UNTIL_START / 6;

                if (tick >= ticks_until_start_sixth && tick <= ticks_until_start_sixth * 2)
                    || (tick >= ticks_until_start_sixth * 3 && tick <= ticks_until_start_sixth * 4)
                    || (tick >= ticks_until_start_sixth * 5 && tick <= ticks_until_start_sixth * 6)
                {
                    ball.draw_arrow(&mut canvas);
                } else if tick > TICKS_UNTIL_START {
                    ball.x += ball.x_velocity;
                    ball.y += ball.y_velocity;
                }

                scoreboard.draw_scoreboard(&mut canvas);
            }

            if paddles.left + paddles.left_velocity > -Paddles::BOTTOM {
                paddles.left_velocity = 0;
            }
            if paddles.left + paddles.left_velocity < -Paddles::TOP {
                paddles.left_velocity = 0;
            }

            if paddles.right + paddles.right_velocity > -Paddles::BOTTOM {
                paddles.right_velocity = 0;
            }
            if paddles.right + paddles.right_velocity < -Paddles::TOP {
                paddles.right_velocity = 0;
            }

            if paddles.ball_does_collide_with_left_paddle(&ball) {
                if ball.x_velocity < 0 {
                    ball.x -= ball.x_velocity;
                    ball.x_velocity *= -1;
                }
            } else if paddles.ball_does_collide_with_right_paddle(&ball) {
                if ball.x_velocity > 0 {
                    ball.x -= ball.x_velocity;
                    ball.x_velocity *= -1;
                }
            }

            if ball.y < (PADDING + Ball::RADIUS) {
                ball.y = PADDING + Ball::RADIUS;
                ball.y_velocity *= -1;
            } else if ball.y > WINDOW_HEIGHT - PADDING - Ball::RADIUS {
                ball.y = WINDOW_HEIGHT - PADDING - Ball::RADIUS;
                ball.y_velocity *= -1;
            }

            if ball.x < Ball::RADIUS * 2 {
                tick = 0;
                scoreboard.right_score += 1;
                ball.reset();
            } else if ball.x > WINDOW_WIDTH - Ball::RADIUS * 2 {
                tick = 0;
                scoreboard.left_score += 1;
                ball.reset();
            }

            paddles.draw_paddles(&mut canvas);
            ball.draw_ball(&mut canvas);
        }

        let center = WINDOW_HEIGHT / 2 - Ball::RADIUS / 2;
        if !show_welcome {
            canvas.set_draw_color(Color::WHITE);
            canvas
                .draw_line(Point::new(center, 0), Point::new(center, WINDOW_HEIGHT))
                .unwrap();
            canvas.set_draw_color(Color::BLACK);
        }

        canvas.present();

        let elapsed_ms = tick_instant.elapsed().as_millis();
        let time_until_next_tick: i128 = TICK_DURATION - i128::try_from(elapsed_ms).unwrap();
        if time_until_next_tick < 0 {
            continue;
        }

        sleep(Duration::from_millis(
            time_until_next_tick.try_into().unwrap(),
        ));
    }
}
