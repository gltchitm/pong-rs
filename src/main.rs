mod ball;
mod paddles;
mod welcome;
mod constants;
mod scoreboard;
mod pressed_keys;

use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::event::Event;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(
        constants::WINDOW_TITLE,
        constants::WINDOW_WIDTH as u32,
        constants::WINDOW_HEIGHT as u32
    ).position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut paddles = paddles::Paddles {
        left: paddles::Paddles::CENTER,
        right: paddles::Paddles::CENTER,
        left_movement: 0,
        right_movement: 0
    };
    let mut ball = ball::Ball {
        x: ball::Ball::X_CENTER,
        y: ball::Ball::Y_CENTER,
        x_movement: ball::Ball::rand_movement(),
        y_movement: ball::Ball::rand_movement()
    };
    let mut scoreboard = scoreboard::Scoreboard {
        left_score: 0,
        right_score: 0
    };
    let mut pressed_keys = pressed_keys::PressedKeys {
        w: false,
        s: false,
        up: false,
        down: false
    };

    let mut show_welcome = true;

    'pong: loop {
        canvas.clear();

        if !show_welcome {
            paddles.left += paddles.left_movement;
            paddles.right += paddles.right_movement;
            paddles.draw_paddles(&mut canvas);
    
            ball.x += ball.x_movement;
            ball.y += ball.y_movement;
            ball.draw_ball(&mut canvas);

            scoreboard.draw_scoreboard(&mut canvas);
        } else {
            welcome::draw_welcome(&mut canvas);
        }
                
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'pong
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    if show_welcome {
                        show_welcome = false;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    if !show_welcome {
                        pressed_keys.w = true;
                        paddles.left_movement = constants::MOVEMENT_UP;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    if !show_welcome {
                        pressed_keys.s = true;
                        paddles.left_movement = constants::MOVEMENT_DOWN;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    if !show_welcome {
                        pressed_keys.up = true;
                        paddles.right_movement = constants::MOVEMENT_UP;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    if !show_welcome {
                        pressed_keys.down = true;
                        paddles.right_movement = constants::MOVEMENT_DOWN;
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::W), .. } => pressed_keys.w = false,
                Event::KeyUp { keycode: Some(Keycode::S), .. } => pressed_keys.s = false,
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => pressed_keys.up = false,
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => pressed_keys.down = false,
                _ => {}
            }
        }
        
        if !show_welcome {
            if !pressed_keys.w && !pressed_keys.s {
                paddles.left_movement = 0;
            } else if pressed_keys.w && !pressed_keys.s {
                paddles.left_movement = constants::MOVEMENT_UP;
            } else if pressed_keys.s && !pressed_keys.w {
                paddles.left_movement = constants::MOVEMENT_DOWN;
            }

            if !pressed_keys.up && !pressed_keys.down {
                paddles.right_movement = 0;
            } else if pressed_keys.up && !pressed_keys.down {
                paddles.right_movement = constants::MOVEMENT_UP;
            } else if pressed_keys.down && !pressed_keys.up {
                paddles.right_movement = constants::MOVEMENT_DOWN;
            }

            if paddles.left + paddles.left_movement > paddles::Paddles::BOTTOM {
                paddles.left_movement = 0;
            }
            if paddles.left + paddles.left_movement < paddles::Paddles::TOP {
                paddles.left_movement = 0;
            }
            
            if paddles.right + paddles.right_movement > paddles::Paddles::BOTTOM {
                paddles.right_movement = 0;
            }
            if paddles.right + paddles.right_movement < paddles::Paddles::TOP {
                paddles.right_movement = 0;
            }

            if paddles.ball_does_collide_with_left_paddle(&ball) {
                if ball.x_movement < 0 {
                    ball.x -= ball.x_movement;
                    ball.x_movement *= -1;
                }
            } else if paddles.ball_does_collide_with_right_paddle(&ball) {
                if ball.x_movement > 0 {
                    ball.x -= ball.x_movement;
                    ball.x_movement *= -1;
                }
            }

            if ball.y < (constants::PADDING + ball::Ball::RADIUS) {
                ball.y = constants::PADDING + ball::Ball::RADIUS;
                ball.y_movement *= -1;
            } else if ball.y > constants::WINDOW_HEIGHT - constants::PADDING - ball::Ball::RADIUS {
                ball.y = constants::WINDOW_HEIGHT - constants::PADDING - ball::Ball::RADIUS;
                ball.y_movement *= -1;
            }

            if ball.x < ball::Ball::RADIUS * 2 {
                scoreboard.right_score += 1;
                ball.reset();
            } else if ball.x > constants::WINDOW_WIDTH - ball::Ball::RADIUS * 2 {
                scoreboard.left_score += 1;
                ball.reset();
            }
        }

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000 / constants::FPS as u32));
    }
}