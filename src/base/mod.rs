use point::Offset;

pub(crate) mod tetromino;
pub(crate) mod matrix;
pub(crate) mod piece;
pub(crate) mod point;
pub(crate) mod window;

impl From<(i8, i8)> for Offset {
    fn from((y, x): (i8, i8)) -> Self {
        Self{row_offset: y as i16, col_offset: x as i16}
    }
}