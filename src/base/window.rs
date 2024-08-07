use crate::{
    base::point::Point,
    const_vals::{DOUBLE_SPACE, WHITESPACE},
    style::BorderStyle,
};

static mut BORDER_STYLE: BorderStyle = BorderStyle::SingleLine;

pub fn set_border_style(style: BorderStyle) {
    unsafe { BORDER_STYLE = style }
}

fn border_style() -> BorderStyle {
    unsafe { BORDER_STYLE }
}

pub(crate) struct Window {
    title: &'static str,
    start: Point,
    end: Point,
}

impl Window {
    pub(crate) const fn new_const(
        top: i16,
        left: i16,
        width: i16,
        height: i16,
        title: &'static str,
    ) -> Self {
        Self {
            title,
            start: Point::new_with_b2c(top, left),
            end: Point::new_const(height - 1, width - 1),
        }
    }

    pub(crate) fn composite_empty(&self) -> String {
        let mut ret = String::with_capacity(64);
        for r in 0..=self.end.row {
            ret.push_str(&self.start.add_row_offset(r).to_moving_string());
            for c in 0..=self.end.col {
                let symbol = if r == 0 || r == self.end.row {
                    if c == 0 {
                        DOUBLE_SPACE
                    } else if c == self.end.col {
                        WHITESPACE
                    } else {
                        DOUBLE_SPACE
                    }
                } else {
                    DOUBLE_SPACE
                };
                ret.push_str(symbol);
            }
        }
        ret
    }
}

impl std::fmt::Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::with_capacity(1024);
        let style = border_style();
        for r in 0..=self.end.row {
            ret.push_str(&self.start.add_row_offset(r).to_moving_string());
            for c in 0..=self.end.col {
                let symbol = if r == 0 {
                    if c == 0 {
                        style.top_left()
                    } else if c == self.end.col {
                        style.top_right()
                    } else {
                        style.h_dash()
                    }
                } else if r == self.end.row {
                    if c == 0 {
                        style.bottom_left()
                    } else if c == self.end.col {
                        style.bottom_right()
                    } else {
                        style.h_dash()
                    }
                } else if c == 0 {
                    style.left_v_dash()
                } else if c == self.end.col {
                    style.right_v_dash()
                } else {
                    DOUBLE_SPACE
                };
                ret.push_str(symbol);
            }
        }

        ret.push_str(
            &(self
                .start
                .add_col_offset(((self.end.col + 1) * 2 - self.title.len() as i16) / 2)
                .to_moving_string()
                + self.title),
        );
        write!(f, "{}", ret)
    }
}
