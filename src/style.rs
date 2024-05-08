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

const WHITESPACE: &str = " ";

const SINGLE_LINE: StyleData = StyleData {
    top_left: "┌",
    top_right: "┐",
    bottom_left: "└",
    bottom_right: "┘",
    vertical_bar: "│",
    dash: "──",
};
const DOUBLE_LINE: StyleData = StyleData {
    top_left: "╔",
    top_right: "╗",
    bottom_left: "╚",
    bottom_right: "╝",
    vertical_bar: "║",
    dash: "══",
};
const THICKER_LINE: StyleData = StyleData {
    top_left: "┏",
    top_right: "┓",
    bottom_left: "┗",
    bottom_right: "┛",
    vertical_bar: "┃",
    dash: "━━",
};
const ROUNDED_LINE: StyleData = StyleData {
    top_left: "╭",
    top_right: "╮",
    bottom_left: "╰",
    bottom_right: "╯",
    vertical_bar: "│",
    dash: "──",
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

    pub(crate) fn top_left(&self) -> String {
        WHITESPACE.to_owned() + self.data().top_left
    }

    pub(crate) fn bottom_left(&self) -> String {
        WHITESPACE.to_owned() + self.data().bottom_left
    }

    pub(crate) fn top_right(&self) -> String {
        self.data().top_right.to_owned()
    }

    pub(crate) fn bottom_right(&self) -> String {
        self.data().bottom_right.to_owned()
    }

    pub(crate) fn left_vertical_bar(&self) -> String {
        WHITESPACE.to_owned() + self.data().vertical_bar
    }

    pub(crate) fn right_vertical_bar(&self) -> String {
        self.data().vertical_bar.to_owned()
    }

    pub(crate) fn dash(&self) -> String {
        self.data().dash.to_owned()
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct StyleData {
    pub top_left: &'static str,
    pub top_right: &'static str,
    pub bottom_left: &'static str,
    pub bottom_right: &'static str,
    pub vertical_bar: &'static str,
    pub dash: &'static str,
}
