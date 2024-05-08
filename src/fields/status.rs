use crate::{base::point::Point, draw};

use super::play;

pub(crate) const FPS_POINT: Point = Point::new_const(10, 4);
pub(crate) const FPS_DATA_POINT: Point = Point::new_const(10, 8);
pub(crate) const LEVEL_POINT: Point = Point::new_const(12, 4);
pub(crate) const LEVEL_DATA_POINT: Point = Point::new_const(12, 10);
pub(crate) const SCORE_POINT: Point = Point::new_const(13, 4);
pub(crate) const SCORE_DATA_POINT: Point = Point::new_const(13, 10);
pub(crate) const LINES_POINT: Point = Point::new_const(14, 4);
pub(crate) const LINES_DATA_POINT: Point = Point::new_const(14, 10);

static mut SCORE: &mut u64 = &mut 0;
static mut LEVEL: &mut u64 = &mut 0;
static mut LINES: &mut u64 = &mut 0;

pub(crate) enum DropKind {
    Soft = 1,
    Hard = 2,
}

pub(crate) fn reset() {
    unsafe {
        *SCORE = 0;
        *LEVEL = 1;
        *LINES = 0;
    }
}

pub(crate) fn add_lines(lines: u64) {
    unsafe {
        *SCORE += match lines {
            1 => 100 * *LEVEL,
            2 => 300 * *LEVEL,
            3 => 500 * *LEVEL,
            4 => 800 * *LEVEL,
            _ => panic!("This should not happened!!!"),
        };
        *LINES += lines
    }
}

pub(crate) fn levelup() {
    unsafe {
        *LEVEL = *LINES / 10 + 1;
        play::set_frame_duration(*LEVEL);
    }
}

pub(crate) fn add_drop_points(drop_kind: DropKind) {
    unsafe { *SCORE += drop_kind as u64 }
}

pub(crate) fn render_score_datas() {
    let level_overwrite = LEVEL_DATA_POINT.add_col_offset(1).to_moving_string() + "  ";
    let level_data = LEVEL_DATA_POINT.composite(unsafe { *LEVEL });

    let score_overwrite = SCORE_DATA_POINT.add_col_offset(1).to_moving_string() + "      ";
    let score_data = SCORE_DATA_POINT.composite(unsafe { *SCORE });

    let lines_overwrite = LINES_DATA_POINT.add_col_offset(1).to_moving_string() + "   ";
    let lines_data = LINES_DATA_POINT.composite(unsafe { *LINES });
    #[cfg(feature = "with_log")]
    crate::logger::log_to_file(
        format_args!(
            "score datas len = {}",
            level_overwrite.as_bytes().len()
                + level_data.as_bytes().len()
                + score_overwrite.as_bytes().len()
                + score_data.as_bytes().len()
                + lines_overwrite.as_bytes().len()
                + lines_data.as_bytes().len()
        ),
        Some("string_lens"),
    );
    draw::render(
        level_overwrite
            + &level_data
            + &score_overwrite
            + &score_data
            + &lines_overwrite
            + &lines_data,
    );
}

pub(crate) fn render_fps(fps: u32) {
    let fps_overwrite = FPS_DATA_POINT.add_col_offset(2).to_moving_string() + " ";
    let new_fps = FPS_DATA_POINT.to_moving_string() + &fps.to_string();
    #[cfg(feature = "with_log")]
    crate::logger::log_to_file(
        format_args!(
            "fps write content len = {}",
            fps_overwrite.as_bytes().len() + new_fps.as_bytes().len()
        ),
        Some("string_lens"),
    );
    draw::render(fps_overwrite);
    draw::render(new_fps);
}

pub(crate) fn render_static_texts() {
    draw::render(format_args!(
        "{}{}{}{}",
        FPS_POINT.composite("FPS:"),
        LEVEL_POINT.composite("Level:"),
        SCORE_POINT.composite("Score:"),
        LINES_POINT.composite("Lines:")
    ))
}
