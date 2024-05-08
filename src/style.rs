/*
// # Box Drawing(Official Unicode Consortium code chart)
//
//     |        |    0   |    1   |    2   |    3   |   4    |   5    |   6    |   7    |   8    |   9    |   A    |   B    |   C    |   D    |   E    |   F    |
//     | :----: | :----: | :----: | :----: | :----: | :----: | :----: | :----: | :----: | :----: | :----: | :----: | :----: | :----: | :----: | :----: | :----: |
//     | U+250x |    ─   |    ━   │    │   |    ┃   |   ┄    |   ┅    |   ┆    |   ┇    |   ┈    |   ┉    |   ┊    |   ┋    |   ┌    |   ┍    |   ┎    |   ┏    |
//     | U+251x |    ┐   |    ┑   |    ┒   |    ┓   |   └    |   ┕    |   ┖    |   ┗    |   ┘    |   ┙    |   ┚    |   ┛    |   ├    |   ┝    |   ┞    |   ┟    |
//     | U+252x |    ┠   |    ┡   |    ┢   |    ┣   |   ┤    |   ┥    |   ┦    |   ┧    |   ┨    |   ┩    |   ┪    |   ┫    |   ┬    |   ┭    |   ┮    |   ┯    |
//     | U+253x |    ┰   |    ┱   |    ┲   |    ┳   |   ┴    |   ┵    |   ┶    |   ┷    |   ┸    |   ┹    |   ┺    |   ┻    |   ┼    |   ┽    |   ┾    |   ┿    |
//     | U+254x |    ╀   |    ╁   |    ╂   |    ╃   |   ╄    |   ╅    |   ╆    |   ╇    |   ╈    |   ╉    |   ╊    |   ╋    |   ╌    |   ╍    |   ╎    |   ╏    |
//     | U+255x |    ═   |    ║   |    ╒   |    ╓   |   ╔    |   ╕    |   ╖    |   ╗    |   ╘    |   ╙    |   ╚    |   ╛    |   ╜    |   ╝    |   ╞    |   ╟    |
//     | U+256x |    ╠   |    ╡   |    ╢   |    ╣   |   ╤    |   ╥    |   ╦    |   ╧    |   ╨    |   ╩    |   ╪    |   ╫    |   ╬    |   ╭    |   ╮    |   ╯    |
//     | U+257x |    ╰   |    ╱   |    ╲   |    ╳   |   ╴    |   ╵    |   ╶    |   ╷    |   ╸    |   ╹    |   ╺    |   ╻    |   ╼    |   ╽    |   ╾    |   ╿    |
*/


const SINGLE_LINE: StyleData = StyleData {
    top_left: " ┌",
    top_right: "┐",
    bottom_left: " └",
    bottom_right: "┘",
    right_v_dash: "│",
    left_v_dash: " │",
    h_dash: "──",
};
const DOUBLE_LINE: StyleData = StyleData {
    top_left: " ╔",
    top_right: "╗",
    bottom_left: " ╚",
    bottom_right: "╝",
    right_v_dash: "║",
    left_v_dash: " ║",
    h_dash: "══",
};
const THICKER_LINE: StyleData = StyleData {
    top_left: " ┏",
    top_right: "┓",
    bottom_left: " ┗",
    bottom_right: "┛",
    right_v_dash: "┃",
    left_v_dash: " ┃",
    h_dash: "━━",
};
const ROUNDED_LINE: StyleData = StyleData {
    top_left: " ╭",
    top_right: "╮",
    bottom_left: " ╰",
    bottom_right: "╯",
    right_v_dash: "│",
    left_v_dash: " │",
    h_dash: "──",
};

///`Style1`: Default style which using single line: ┌ ┐ │ └ ┘ ──
///
///`Style2`: The style using double line: ╔ ╗ ║ ╚ ╝ ══
///
///`Style3`: The style using thicker line: ┏ ┓ ┃ ┗ ┛ ━━
///
/// `Style4`: The style using single line and rounded corners: ╭ ╮ │ ╰ ╯ ──
#[derive(Debug, Clone, Copy, Default)]
pub enum BorderStyle {
    #[default]
    SingleLine,
    DoubleLine,
    ThickerLine,
    RoundedCorners,
}

impl BorderStyle {
    fn data(&self) -> &StyleData {
        match self {
            BorderStyle::SingleLine => &SINGLE_LINE,
            BorderStyle::DoubleLine => &DOUBLE_LINE,
            BorderStyle::ThickerLine => &THICKER_LINE,
            BorderStyle::RoundedCorners => &ROUNDED_LINE,
        }
    }

    pub(crate) fn top_left(&self) -> &str {
        self.data().top_left
    }

    pub(crate) fn bottom_left(&self) -> &str {
        self.data().bottom_left
    }

    pub(crate) fn top_right(&self) -> &str {
        self.data().top_right
    }

    pub(crate) fn bottom_right(&self) -> &str {
        self.data().bottom_right
    }

    pub(crate) fn left_v_dash(&self) -> &str {
        self.data().left_v_dash
    }

    pub(crate) fn right_v_dash(&self) -> &str {
        self.data().right_v_dash
    }

    pub(crate) fn h_dash(&self) -> &str {
        self.data().h_dash
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct StyleData {
    pub top_left: &'static str,
    pub top_right: &'static str,
    pub bottom_left: &'static str,
    pub bottom_right: &'static str,
    pub right_v_dash: &'static str,
    pub left_v_dash: &'static str,
    pub h_dash: &'static str,
}

#[cfg(test)]
mod tests {
    use crate::style::{DOUBLE_LINE, ROUNDED_LINE, SINGLE_LINE, THICKER_LINE};

    #[test]
    fn test_style_size() {
        println!("single line size: top_left = {}, top_right = {}, bottom_left = {}, bottom_right = {}, v_dash = {}, h_dash = {}",  
        SINGLE_LINE.top_left.len(), SINGLE_LINE.top_right.len(), SINGLE_LINE.bottom_left.len(), SINGLE_LINE.bottom_right.len(), SINGLE_LINE.right_v_dash.len(), SINGLE_LINE.h_dash.len());
        println!("double line size: top_left = {}, top_right = {}, bottom_left = {}, bottom_right = {}, v_dash = {}, h_dash = {}",  
        DOUBLE_LINE.top_left.len(), DOUBLE_LINE.top_right.len(), DOUBLE_LINE.bottom_left.len(), DOUBLE_LINE.bottom_right.len(), DOUBLE_LINE.right_v_dash.len(), DOUBLE_LINE.h_dash.len());
        println!("thicker line size: top_left = {}, top_right = {}, bottom_left = {}, bottom_right = {}, v_dash = {}, h_dash = {}",  
        THICKER_LINE.top_left.len(), THICKER_LINE.top_right.len(), THICKER_LINE.bottom_left.len(), THICKER_LINE.bottom_right.len(), THICKER_LINE.right_v_dash.len(), THICKER_LINE.h_dash.len());
        println!("rounded line size: top_left = {}, top_right = {}, bottom_left = {}, bottom_right = {}, v_dash = {}, h_dash = {}",  
        ROUNDED_LINE.top_left.len(), ROUNDED_LINE.top_right.len(), ROUNDED_LINE.bottom_left.len(), ROUNDED_LINE.bottom_right.len(), ROUNDED_LINE.right_v_dash.len(), ROUNDED_LINE.h_dash.len());
    }
}
