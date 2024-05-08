#![allow(unused_assignments)]
use std::{io::Write, sync::OnceLock, time::Duration};

use base::{
    piece::Piece,
    point::PointState,
    tetromino::{TetroState, Tetromino, TetrominoColor},
};

use const_vals::{GAMEOVER_TEXT_POINT, GAMEOVER_WINDOW, NUM_PLAYGROUND_ROWS, RESTART_TIP_POINT};
use fields::{info, next_queue, play, status};
use style::BorderStyle;
use term::TermColorful;

use crate::fields::hold_chamber;

mod util;

/// base types
pub(crate) mod base;
/// render
pub(crate) mod draw;
/// simple border styles
pub mod style;

/// terminal control
pub(crate) mod term;

/// most of the constants that can be modified as needed
pub(crate) mod const_vals;
/// game control
pub(crate) mod control;

/// hold chamber, status, play field, next queue field and help tip field
pub(crate) mod fields;
#[cfg(feature = "with_log")]
mod logger;

#[derive(Debug)]
pub struct Tetris {
    running: std::sync::atomic::AtomicBool,
    current_piece: Piece,
    loop_duration: Duration,
    gameovered: bool,
    helping_on_gameovered: bool,
    helping: bool,
    reseted: bool,
    listener_handle: Option<std::thread::JoinHandle<()>>,
}

impl Tetris {
    pub fn instance() -> &'static mut Tetris {
        static mut TETRIS: OnceLock<Tetris> = OnceLock::new();
        unsafe {
            match TETRIS.get_mut() {
                Some(t) => t,
                None => {
                    #[cfg(feature = "test_wallkick")]
                    let current_piece = Piece::new(Tetromino::new(base::tetromino::TetrominoKind::J));

                    #[cfg(not(feature = "test_wallkick"))]
                    let current_piece = Piece::new(next_queue::take_tetromino());
                    TETRIS
                        .set(Self {
                            current_piece,
                            running: std::sync::atomic::AtomicBool::new(false),
                            loop_duration: Self::get_loop_duration(120),
                            helping_on_gameovered: false,
                            gameovered: false,
                            reseted: false,
                            helping: false,
                            listener_handle: None,
                        })
                        .unwrap();
                    TETRIS.get_mut().unwrap()
                }
            }
        }
    }

    pub fn set_max_fps(&mut self, max_fps: u64) -> &mut Self {
        self.loop_duration = Self::get_loop_duration(max_fps);
        self
    }

    pub fn set_border_style(&mut self, border_style: BorderStyle) -> &mut Self {
        base::window::set_border_style(border_style);
        self
    }

    pub fn start(&mut self) {
        draw::render_basic_windows();
        self.set_running(true);
        self.listener_handle = Some(std::thread::spawn(control::readkey_event));
        status::render_static_texts();
        draw::render(info::text());
        term::stdout().flush().unwrap();
        self.run_loop();
        self.exit();
    }

    fn set_running(&mut self, value: bool) {
        self.running
            .store(value, std::sync::atomic::Ordering::Relaxed)
    }

    fn running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::Relaxed)
    }

    fn run_loop(&mut self) {
        let mut fps = 0_u32;
        while self.running() {
            if self.gameovered {
                let s: String = GAMEOVER_WINDOW.to_string()
                    + &GAMEOVER_TEXT_POINT.to_moving_string()
                    + &" Game Over!".with_fg(TetrominoColor::Red.to_id())
                    + &RESTART_TIP_POINT.to_moving_string()
                    + &" [r]estart?".with_fg(TetrominoColor::Red.to_id());
                if self.helping_on_gameovered {
                    play::render(self.reseted);
                    self.helping_on_gameovered =false;
                }
                draw::render(s)
            } else {
                self.process();
                next_queue::render();
                play::render(self.reseted);
                hold_chamber::render();
                status::render_score_datas();
                status::render_fps(fps);
            }
            if self.helping {
                if self.gameovered {
                    draw::render(GAMEOVER_WINDOW.composite_empty());
                    self.helping_on_gameovered = true;
                }
                draw::render_help();
            }
            term::stdout().flush().unwrap();
            fps = util::fps_count();
            std::thread::sleep(self.loop_duration);
        }
    }

    pub(crate) fn process(&mut self) {
        play::update_frame(&self.current_piece);
        if crate::util::timer(play::frame_duration()) {
            if self.current_piece.down() {
                return;
            }
            if self.current_piece.locking() {
                self.current_piece.merge_to_matrix(play::playfield());
                self.clear_line();

                let t = next_queue::take_tetromino();
                if !self.try_set_current_piece(t) {
                    return;
                }
                self.current_piece.set_locking(false);
                hold_chamber::reset_swapped();
                self.reseted = false;
            } else {
                self.current_piece.set_locking(true);
            }
        }
    }

    fn swap_hold(&mut self) {
        if hold_chamber::swapped() {
            return;
        }
        let mut swappable = false;
        let mut curr_tetro = self.current_piece.tetromino();
        curr_tetro.set_state(Default::default());
        if let Some(mut next_tetro) = hold_chamber::replace_hold(curr_tetro) {
            for _ in TetroState::RANGE {
                if next_tetro.check_swappable(self.current_piece.center()) {
                    swappable = true;
                    break;
                } else {
                    next_tetro.set_state(next_tetro.state() + 1);
                }
            }
            if !swappable {
                self.gameovered = true;
                return;
            }
            self.current_piece.set_tetromino(next_tetro);
        } else if !self.try_set_current_piece(next_queue::take_tetromino()) {
            return;
        }
        hold_chamber::set_swapped();
    }

    #[inline(always)]
    fn try_set_current_piece(&mut self, tetro: Tetromino) -> bool {
        if tetro.check_downable() {
            self.current_piece = Piece::new(tetro);
            true
        } else {
            self.gameovered = true;
            false
        }
    }

    fn exit(&mut self) {
        self.listener_handle
            .take()
            .unwrap()
            .join()
            .unwrap_or_else(|e| {
                eprintln!(
                    "Readkey thread handle join main thread failed!\nReason: {:?}",
                    e
                )
            });
        draw::render_exiting();
    }

    fn down(&mut self) {
        if self.current_piece.down() {
            status::add_drop_points(crate::fields::status::DropKind::Soft);
        }
    }

    #[inline]
    fn get_loop_duration(max_fps: u64) -> Duration {
        const X: f64 = -17.2;
        const Y: f64 = 249.0;
        const DEFAULT_LOOP_DUR: Duration = Duration::from_nanos(8546540);
        let dur_milliseconds = (max_fps as f64 - Y) / X;
        if dur_milliseconds > 0.0 {
            Duration::from_micros((dur_milliseconds * 1000_f64) as u64)
        } else {
            DEFAULT_LOOP_DUR
        }
    }

    fn drop(&mut self) {
        while self.current_piece.down() {
            status::add_drop_points(crate::fields::status::DropKind::Hard);
        }
        self.current_piece.set_locking(true);
    }

    fn clear_line(&mut self) {
        let mut line_count = 0;
        let mut y = 0;
        while y < NUM_PLAYGROUND_ROWS {
            let mut full = true;
            for cell in play::playfield()[y] {
                if let PointState::Empty = cell {
                    full = false;
                    break;
                }
            }
            if full {
                play::remove_and_push_to_playground(y);
                line_count += 1;
            } else {
                y += 1
            }
        }
        if line_count != 0 {
            status::add_lines(line_count);
            status::levelup();
        }
    }

    fn help(&mut self) {
        self.helping = !self.helping;
        self.reseted = !self.helping;
    }

    fn reset(&mut self) {
        play::reset();
        self.current_piece = Piece::new(next_queue::take_tetromino());
        #[cfg(feature = "test_wallkick")]
        preload_ground();
        hold_chamber::reset();
        status::reset();
        self.gameovered = false;
        self.reseted = true;
    }

    fn rotate(&mut self, state: base::tetromino::TetroState) {
        self.current_piece.rotate(state);
    }

    fn right(&mut self) {
        self.current_piece.right();
    }

    fn left(&mut self) {
        self.current_piece.left();
    }
}

#[cfg(feature = "test_wallkick")]
fn preload_ground() {

    let mut map_file_path = std::env::current_dir().unwrap();
    map_file_path.push("tetris.map");
    let file_path = map_file_path.clone();
    if let Ok(f) = std::fs::File::open(file_path) {
        let mut bufreader = std::io::BufReader::new(f);
        for row in crate::fields::play::playfield().iter_mut().take(20).rev() {
            let mut line = String::with_capacity(10);
            std::io::BufRead::read_line(&mut bufreader, &mut line).unwrap();
            for (i, cell) in row.iter_mut().enumerate() {
                if line.as_bytes()[i] == b'X' {
                    *cell = base::point::PointState::Color(base::tetromino::TetrominoColor::Gray);
                }
            }
        }
    }
}
