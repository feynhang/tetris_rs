use std::ops::{Add, AddAssign, Sub};

use crate::{
    const_vals::DOUBLE_SPACE,
    term::{TermColorful, CSI},
};

use super::matrix;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Point {
    pub(crate) row: i16,
    pub(crate) col: i16,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Offset {
    pub(crate) row_offset: i16,
    pub(crate) col_offset: i16,
}

impl Sub<Offset> for Offset {
    type Output = Self;

    fn sub(self, rhs: Offset) -> Self::Output {
        Self {
            row_offset: self.row_offset - rhs.row_offset,
            col_offset: self.col_offset - rhs.col_offset,
        }
    }
}

impl Add<Offset> for Point {
    type Output = Point;

    fn add(self, rhs: Offset) -> Self::Output {
        Self {
            row: self.row + rhs.row_offset,
            col: self.col + rhs.col_offset,
        }
    }
}

impl AddAssign<Offset> for Point {
    fn add_assign(&mut self, rhs: Offset) {
        *self = self.add(rhs)
    }
}

impl Offset {
    pub(crate) const fn new(col_offset: i16, row_offset: i16) -> Self {
        Self {
            row_offset,
            col_offset,
        }
    }
}

impl Point {
    // pub(crate) const ORIGIN: Point = Point::new_const(0, 0);

    pub(crate) fn new(row: i16, col: i16) -> Self {
        Self { row, col }
    }

    pub(crate) const fn new_const(row: i16, col: i16) -> Self {
        Self { row, col }
    }

    pub(crate) const fn new_with_b2c(row: i16, col: i16) -> Self {
        Self {
            row,
            col: Self::convert_b2c(col),
        }
    }

    fn add_then_b2c(&self, row_offset: i16, col_offset: i16) -> Self {
        Self {
            row: self.row + row_offset,
            col: Self::convert_b2c(self.col + col_offset),
        }
    }

    pub(crate) fn out_of_bound(&self, max_row_idx: i16, max_col_idx: i16) -> bool {
        self.row < 0 || self.row > max_row_idx || self.col < 0 || self.col > max_col_idx
    }

    pub(crate) fn add_row_offset(&self, row_offset: i16) -> Point {
        Self {
            row: self.row + row_offset,
            col: self.col,
        }
    }

    pub(crate) fn add_col_offset(&self, col_offset: i16) -> Point {
        Self {
            col: self.col + col_offset,
            row: self.row,
        }
    }

    #[inline(always)]
    fn update_buffer<const SIZE_COLS: usize>(
        buffer: matrix::MutMatrixSlice<'_, SIZE_COLS>,
        point: Point,
        point_state: &PointState,
    ) -> bool {
        if buffer[point] == *point_state {
            return false;
        }
        buffer[point] = *point_state;
        true
    }

    pub(crate) fn composite_matrix<const SIZE_COLS: usize>(
        &self,
        matrix: matrix::MatrixSlice<'_, SIZE_COLS>,
        buffer: matrix::MutMatrixSlice<'_, SIZE_COLS>,
        blank_optional: Option<&'static str>,
        is_hold: bool,
    ) -> String {
        let mut ret = String::with_capacity(512);
        let s_blank = if let Some(blank) = blank_optional {
            blank
        } else {
            DOUBLE_SPACE
        };
        for (y, cols) in matrix.iter().enumerate() {
            for (x, cell) in cols.iter().enumerate() {
                let curr_point = Point::new(y as i16, x as i16);
                if !Self::update_buffer(buffer, curr_point, cell) {
                    continue;
                }
                let point_state_string = match *cell {
                    PointState::Empty => s_blank.to_owned(),
                    PointState::Color(color) => {
                        let color_str = DOUBLE_SPACE.with_bg(color.to_id());
                        if is_hold {
                            Point::new(0, -1).to_moving_string() + &color_str
                        } else {
                            color_str
                        }
                    }
                    PointState::Shadow(color_id) => "\u{25e3}\u{25e5}".with_fg(color_id),
                    PointState::Uninit => continue,
                };
                let dest_point = self.add_then_b2c(matrix.len() as i16 - y as i16 - 1, x as i16);
                // if is_hold {
                //     dest_point = dest_point.add_col_offset(-1);
                // }
                ret.push_str(&(dest_point.to_moving_string() + &point_state_string));
            }
        }
        ret
    }

    pub(crate) fn composite<T: std::fmt::Display>(self, another_data: T) -> String {
        format!("{}{}", self.to_moving_string(), another_data)
    }

    pub(crate) fn to_moving_string(self) -> String {
        format!("{}{};{}H", CSI, self.row, self.col)
    }

    #[inline(always)]
    const fn convert_b2c(b_col: i16) -> i16 {
        2 * b_col - 1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) enum PointState {
    Uninit,
    #[default]
    Empty,
    Color(super::tetromino::TetrominoColor),
    Shadow(u8),
}

impl std::fmt::Display for PointState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}
