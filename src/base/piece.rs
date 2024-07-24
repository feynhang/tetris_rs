use crate::{
    base::{
        matrix,
        point::{Offset, Point, PointState},
        tetromino::Tetromino,
    },
    const_vals::PIECE_INIT_POINT,
};

use super::tetromino::TetroState;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum PieceState {
    #[default]
    Normal,
    Ghost,
    Unavailable,
}

#[derive(Debug, Clone)]
pub(crate) struct Piece {
    tetromino: Tetromino,
    center: Point,
    status: PieceState,
    locking: bool,
}

impl Piece {
    pub(crate) fn new(t: Tetromino) -> Self {
        Self {
            tetromino: t,
            center: PIECE_INIT_POINT,
            status: Default::default(),
            locking: false,
        }
    }
    pub(crate) fn new_with_center(t: Tetromino, center: Point) -> Self {
        Self {
            tetromino: t,
            center,
            status: Default::default(),
            locking: false,
        }
    }

    pub(crate) fn set_locking(&mut self, value: bool) {
        self.locking = value
    }

    pub(crate) fn locking(&self) -> bool {
        self.locking
    }

    pub(crate) fn set_status(&mut self, piece_status: PieceState) {
        self.status = piece_status
    }

    pub(crate) fn center(&self) -> Point {
        self.center
    }

    #[inline(always)]
    pub(crate) fn merge_to_matrix<const SIZE_COLS: usize>(
        &self,
        mut_matrix_slice: matrix::MutMatrixSlice<'_, SIZE_COLS>,
    ) {
        let center = self.center;
        for offset in self.tetromino.data() {
            let curr_point = center + offset.into();

            if let PointState::Empty = mut_matrix_slice[curr_point] {
                mut_matrix_slice[curr_point] = self.to_point_state();
            }
        }
    }

    pub(crate) fn tetromino(&self) -> Tetromino {
        self.tetromino
    }

    pub(crate) fn set_tetromino(&mut self, tetro: Tetromino) {
        self.tetromino = tetro
    }

    pub(crate) fn to_point_state(&self) -> PointState {
        match self.status {
            PieceState::Normal => self.tetromino.color_point_state(),
            PieceState::Ghost => self.tetromino.shadow_point_state(),
            PieceState::Unavailable => self.tetromino.unavailable_point_state(),
        }
    }

    pub(crate) fn down(&mut self) -> bool {
        self.move_tetro(Offset::new(0, -1))
    }

    pub(crate) fn left(&mut self) -> bool {
        self.move_tetro(Offset::new(-1, 0))
    }
    pub(crate) fn right(&mut self) -> bool {
        self.move_tetro(Offset::new(1, 0))
    }

    pub(crate) fn rotate(&mut self, add_state: TetroState) -> bool {
        let mut new_tetro = self.tetromino();
        new_tetro.set_state(new_tetro.state() + add_state);
        let old_offsets = self.tetromino.kick_offsets();
        for (idx, new_offset) in new_tetro.kick_offsets().iter().enumerate() {
            let check_offset = Offset::from_pair(*new_offset) - old_offsets[idx];
            if new_tetro.check_movable(self.center + check_offset) {
                self.tetromino = new_tetro;
                self.center += check_offset;
                return true;
            }
        }
        false
    }

    fn move_tetro(&mut self, offset: Offset) -> bool {
        if self.tetromino.check_movable(self.center + offset) {
            self.center += offset;
            true
        } else {
            false
        }
    }
}
