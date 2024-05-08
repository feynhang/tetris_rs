use crate::base::{point::Point, window::Window};
pub(crate) const WHITESPACE: &str = " ";
pub(crate) const DOUBLE_SPACE: &str = "  ";
// pub const FRAME_BLANK: &str = "\u{30FB}";
pub(crate) const FRAME_BLANK: &str = DOUBLE_SPACE;
pub(crate) const NUM_NEXTS_COLS: usize = 6;
pub(crate) const NUM_NEXTS_ROWS: usize = 15;
pub(crate) const NUM_HOLD_ROWS: usize = 4;
pub(crate) const NUM_HOLD_COLS: usize = 7;
pub(crate) const NUM_PLAYGROUND_ROWS: usize = 22;
pub(crate) const NUM_PLAYGROUND_COLS: usize = 10;


pub(crate) const FRAME_INIT_POINT: Point = Point::new_const(2, 11);
pub(crate) const NEXT_INIT_POINT: Point = Point::new_const(2, 23);
pub(crate) const HOLD_INIT_POINT: Point = Point::new_const(2, 2);
pub(crate) const PIECE_INIT_POINT: Point = Point::new_const(20, 4);

pub(crate) const HOLD_TETRO_CENTER: Point = Point::new_const(1, 3);

pub(crate) const GAMEOVER_TEXT_POINT: Point = Point::new_with_b2c(10, 13);
pub(crate) const RESTART_TIP_POINT: Point = Point::new_with_b2c(11, 13);

pub(crate) const INFO_TEXT_POINT: Point = Point::new_with_b2c(21, 23);
pub(crate) const HELP_TEXT_POINT: Point = Point::new_with_b2c(4, 11);

pub(crate) const GAMEOVER_WINDOW: Window = Window::new_const(9, 12, 8, 4, "");
pub(crate) const HOLD_WINDOW: Window = Window::new_const(1, 1, 9, 6, "Hold");
pub(crate) const TETRIS_WINDOW: Window = Window::new_const(1, 10, 12, 22, "Tetris");
pub(crate) const STATUS_WINDOW: Window = Window::new_const(7, 1, 9, 16, "Status");
pub(crate) const INFO_WINDOW: Window = Window::new_const(19, 22, 8, 4, "Info");
pub(crate) const NEXT_WINDOW: Window = Window::new_const(1, 22, 8, 18, "Next");


pub(crate) const EXIT_GREETING: &str = "Bye";
pub(crate) const EXIT_GREETING_COLOR: u8 = 47;
