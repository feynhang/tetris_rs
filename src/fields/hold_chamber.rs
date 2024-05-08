use crate::{
    base::{point::PointState, tetromino::Tetromino},
    const_vals::{HOLD_INIT_POINT, NUM_HOLD_COLS, NUM_HOLD_ROWS},
    draw,
};
type HoldMatrix = crate::base::matrix::Matrix<NUM_HOLD_ROWS, NUM_HOLD_COLS>;
static mut HOLD: &mut Option<Tetromino> = &mut None;
static mut SWAPPED: bool = false;
static mut BUFFER: &mut HoldMatrix = &mut crate::base::matrix::default(PointState::Uninit);

pub(crate) fn swapped() -> bool {
    unsafe { SWAPPED }
}

pub(crate) fn set_swapped() {
    unsafe {
        SWAPPED = true;
    }
}

pub(crate) fn reset_swapped() {
    unsafe {
        SWAPPED = false;
    }
}

pub(crate) fn replace_hold(new_tetromino: Tetromino) -> Option<Tetromino> {
    unsafe { HOLD.replace(new_tetromino) }
}

pub(crate) fn hold() -> &'static Option<Tetromino> {
    unsafe { HOLD }
}

pub(crate) fn reset() {
    unsafe {
        *HOLD = None;
        SWAPPED = false;
        BUFFER.fill([PointState::Uninit; NUM_HOLD_COLS]);
    }
}

pub(crate) fn render() {
    let mut hold_matrix: HoldMatrix = Default::default();
    if let Some(tetromino) = hold() {
        let mut piece = crate::base::piece::Piece::new_with_center(
            *tetromino,
            crate::const_vals::HOLD_TETRO_CENTER,
        );
        if swapped() {
            piece.set_status(crate::base::piece::PieceState::Unavailable);
        }
        piece.merge_to_matrix(&mut hold_matrix);
    }

    let hold_content =
        HOLD_INIT_POINT.composite_matrix(&hold_matrix, unsafe { BUFFER }, None, true);
    #[cfg(feature = "with_log")]
    crate::logger::log_to_file(
        format_args!("hold chamber len = {}", hold_content.as_bytes().len()),
        Some("string_lens"),
    );
    draw::render(hold_content)
}
