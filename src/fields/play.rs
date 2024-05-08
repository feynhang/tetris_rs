use std::time::Duration;

use crate::{
    base::{
        matrix::{self, GameMatrix},
        piece::{Piece, PieceState},
        point::PointState,
    },
    const_vals::{NUM_PLAYGROUND_COLS, NUM_PLAYGROUND_ROWS},
    draw,
};

pub(crate) const NUM_FRAME_ROWS: usize = NUM_PLAYGROUND_ROWS - 2;
static mut FRAME_DUR: Option<Duration> = None;
static mut FRAME: &mut GameMatrix = &mut matrix::default(PointState::Empty);
static mut PLAYGROUND: &mut GameMatrix = &mut matrix::default(PointState::Empty);

pub(crate) fn frame_duration() -> Duration {
    unsafe {
        match FRAME_DUR {
            Some(dur) => dur,
            None => {
                FRAME_DUR = Some(get_frame_duration(1));
                FRAME_DUR.unwrap()
            }
        }
    }
}

pub(crate) fn set_frame_duration(level: u64) {
    unsafe { FRAME_DUR = Some(get_frame_duration(level)) }
}

pub(crate) fn playfield() -> &'static mut GameMatrix {
    unsafe { PLAYGROUND }
}

pub(crate) fn buffer() -> &'static mut GameMatrix {
    static mut BUFFER: &mut GameMatrix = &mut matrix::default(PointState::Uninit);
    unsafe { BUFFER }
}

pub(crate) fn current_frame() -> &'static mut GameMatrix {
    unsafe { FRAME }
}

pub(crate) fn update_frame(piece: &Piece) {
    sync_frame_to_playground();
    // update color piece to current frame
    piece.merge_to_matrix(current_frame());
    let mut ghost = piece.clone();
    ghost.set_status(PieceState::Ghost);
    // update ghost piece to current frame
    while ghost.down() {}
    ghost.merge_to_matrix(current_frame());
}

pub(crate) fn remove_and_push_to_playground(index: usize) {
    unsafe {
        let ptr = playfield().as_mut_ptr().add(index);
        std::ptr::copy(ptr.add(1), ptr, NUM_PLAYGROUND_ROWS - index - 1);
        *playfield().get_unchecked_mut(NUM_PLAYGROUND_ROWS - 1) = Default::default();
    }
}

pub(crate) fn reset() {
    unsafe {
        FRAME.fill_with(Default::default);
        PLAYGROUND.fill_with(Default::default);
        FRAME_DUR = Some(get_frame_duration(1));
        clear_buffer();
    }
}

pub(crate) fn clear_buffer() {
    buffer().fill([PointState::Uninit; NUM_PLAYGROUND_COLS])
}

pub(crate) fn render(reseted: bool) {
    if reseted {
        clear_buffer();
    }
    let content = crate::const_vals::FRAME_INIT_POINT.composite_matrix(
        &current_frame()[0..NUM_FRAME_ROWS],
        buffer(),
        Some(crate::const_vals::FRAME_BLANK),
        false,
    );
    #[cfg(feature = "with_log")]
    crate::logger::log_to_file(
        format_args!("playfield len = {}", content.as_bytes().len()),
        Some("string_lens"),
    );
    draw::render(content)
}

#[inline(always)]
fn sync_frame_to_playground() {
    unsafe { *FRAME = *PLAYGROUND }
}

#[inline(always)]
fn get_frame_duration(level: u64) -> Duration {
    let v = (0.8 - (level as f64 - 1.0) * 0.007).powf(level as f64 - 1.0);
    Duration::from_millis((v * 1000_f64) as u64)
}
