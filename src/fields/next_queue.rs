use std::{collections::VecDeque, sync::OnceLock};

use crate::{
    base::{
        matrix,
        piece::Piece,
        tetromino::{Tetromino, TetrominoKind},
    },
    const_vals::{NUM_NEXTS_COLS, NUM_NEXTS_ROWS},
    draw, util,
};

type NextsMatrix = matrix::Matrix<NUM_NEXTS_ROWS, NUM_NEXTS_COLS>;
const NUM_PREVIEWS: usize = 5;
const ALL_KIND_TETROS: [Tetromino; 7] = [
    Tetromino::new(TetrominoKind::I),
    Tetromino::new(TetrominoKind::J),
    Tetromino::new(TetrominoKind::L),
    Tetromino::new(TetrominoKind::O),
    Tetromino::new(TetrominoKind::S),
    Tetromino::new(TetrominoKind::T),
    Tetromino::new(TetrominoKind::Z),
];

static mut BUFFER: &mut NextsMatrix = &mut matrix::default(crate::base::point::PointState::Uninit);
static mut TETROS_QUEUE: &mut VecDeque<Tetromino> = &mut VecDeque::new();

fn bag() -> &'static mut Vec<Tetromino> {
    static mut BAG: OnceLock<Vec<Tetromino>> = OnceLock::new();
    unsafe {
        match BAG.get_mut() {
            Some(bag) => bag,
            None => {
                BAG.set(ALL_KIND_TETROS.to_vec()).unwrap();
                BAG.get_mut().unwrap()
            }
        }
    }
}

fn tetro_queue() -> &'static mut VecDeque<Tetromino> {
    try_enqueue();
    unsafe {
       TETROS_QUEUE
    }
}

fn try_enqueue() {
    unsafe {
        while TETROS_QUEUE.len() < NUM_PREVIEWS {
            TETROS_QUEUE.push_back(bag().remove(util::rand_in_range(0..bag().len())));
            if bag().is_empty() {
                bag().extend(ALL_KIND_TETROS);
            }
        }
    }
}

pub(crate) fn take_tetromino() -> Tetromino {
    let tetro = tetro_queue().pop_front().unwrap();
    try_enqueue();
    tetro
}

pub(crate) fn render() {
    const COL_NEXTS_PIECE: i16 = 2;
    let mut nexts_ground = NextsMatrix::default();
    let mut y_ccs = 12;
    let mut nexts_data_backup = unsafe { TETROS_QUEUE.clone() };
    while let Some(tetro) = nexts_data_backup.pop_front() {
        Piece::new_with_center(
            tetro,
            crate::base::point::Point::new(y_ccs, COL_NEXTS_PIECE),
        )
        .merge_to_matrix(&mut nexts_ground);
        y_ccs = y_ccs.saturating_sub(3);
    }
    let content = crate::const_vals::NEXTS_INIT_POINT.composite_matrix(
        &nexts_ground,
        unsafe { BUFFER },
        None,
        false,
    );

    #[cfg(feature = "with_log")]
    crate::logger::log_to_file(
        format_args!("next field content len = {}", content.as_bytes().len()),
        Some("string_lens"),
    );
    draw::render(content)
}
