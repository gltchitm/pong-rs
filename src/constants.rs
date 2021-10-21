use sdl2::pixels::Color;

pub const FPS: i32 = 60;
pub const PADDING: i32 = 15;
pub const SCOREBOARD_PADDING: i32 = 28;
pub const SCOREBOARD_FONT_SIZE: i32 = 48;
pub const FONT: &str = "assets/PressStart2P-Regular.ttf";

pub const WINDOW_WIDTH: i32 = 750;
pub const WINDOW_HEIGHT: i32 = 750;
pub const WINDOW_TITLE: &str = "Pong";

const PADDLE_SPEED: i32 = 350;
pub const MOVEMENT_UP: i32 = -PADDLE_SPEED / (FPS / 2);
pub const MOVEMENT_DOWN: i32 = PADDLE_SPEED / (FPS / 2);

const BALL_SPEED: i32 = 150;
pub const BALL_MOVEMENT: i32 = BALL_SPEED / (FPS / 2);

pub const WELCOME_TOP_FONT_SIZE: i32 = 64;
pub const WELCOME_BOTTOM_FONT_SIZE: i32 = 24;
pub const WELCOME_TOP_Y_POS: i32 = WINDOW_WIDTH / 3;
pub const WELCOME_BOTTOM_Y_POS: i32 = WINDOW_WIDTH / 2;
pub const WELCOME_TOP_TEXT: &str = "Pong";
pub const WELCOME_BOTTOM_TEXT: &str = "Press SPACE To Start!";
pub const WELCOME_TEXT_COLOR: Color = Color::RGB(195, 195, 195);
