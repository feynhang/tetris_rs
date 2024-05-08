use std::ops::{Index, IndexMut};

use crate::const_vals::{NUM_PLAYGROUND_COLS, NUM_PLAYGROUND_ROWS};

use super::point::{Point, PointState};

pub(crate) type Columns<const SIZE_COLS: usize> = [PointState; SIZE_COLS];

pub(crate) type Matrix<const SIZE_ROWS: usize, const SIZE_COLS: usize> =
    [Columns<SIZE_COLS>; SIZE_ROWS];

pub(crate) type MatrixSlice<'a, const SIZE_COLS: usize> = &'a [Columns<SIZE_COLS>];

pub(crate) type MutMatrixSlice<'a, const SIZE_COLS: usize> = &'a mut [Columns<SIZE_COLS>];

pub(crate) type GameMatrix = Matrix<NUM_PLAYGROUND_ROWS, NUM_PLAYGROUND_COLS>;

pub(crate) const fn default<const SIZE_ROWS: usize, const SIZE_COLS: usize>(
    point_state: PointState,
) -> Matrix<SIZE_ROWS, SIZE_COLS> {
    [[point_state; SIZE_COLS]; SIZE_ROWS]
}

impl<const SIZE_COLS: usize> Index<Point> for [Columns<SIZE_COLS>] {
    type Output = PointState;

    fn index(&self, point: Point) -> &Self::Output {
        unsafe {
            self.get_unchecked(point.row as usize)
                .get_unchecked(point.col as usize)
        }
    }
}

impl<const SIZE_COLS: usize> IndexMut<Point> for [Columns<SIZE_COLS>] {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        unsafe {
            self.get_unchecked_mut(point.row as usize)
                .get_unchecked_mut(point.col as usize)
        }
    }
}

#[allow(unused)]
#[cfg(feature = "with_log")]
pub(crate) fn matrix_to_string<const SIZE_COLS: usize>(matrix: MatrixSlice<SIZE_COLS>) -> String {
    let mut ret = String::with_capacity(8192);
    ret += "[\n";
    for inner_arr in matrix.iter().rev() {
        ret += &format!("\t{:?},\n", inner_arr);
    }
    ret.push(']');
    ret
}
