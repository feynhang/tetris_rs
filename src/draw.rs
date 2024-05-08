use std::io::Write;

use crate::{
    base::point::Point,
    const_vals::{self, EXIT_GREETING, HELP_TEXT_POINT},
    term::{self, TermColorful},
};
const EXIT_GREETING_POINT: Point = Point::new_const(1, 1);

pub(crate) fn render_basic_windows() {
    term::hide_cursor();
    term::clear_screen();
    render(format_args!(
        "{}{}{}{}{}",
        const_vals::HOLD_WINDOW,
        const_vals::TETRIS_WINDOW,
        const_vals::STATUS_WINDOW,
        const_vals::INFO_WINDOW,
        const_vals::NEXT_WINDOW
    ));
}

pub(crate) fn render_exiting() {
    term::clear_screen();
    term::show_cursor();
    let content =
        EXIT_GREETING_POINT.to_moving_string() + &EXIT_GREETING.with_fg(const_vals::EXIT_GREETING_COLOR);
    render(content);
    term::stdout().flush().unwrap();
}

pub(crate) fn render_help() {
    let help_info = HELP_TEXT_POINT.composite("Help Info:")
        + &HELP_TEXT_POINT.add_row_offset(1).composite("Rotate  [w]")
        + &HELP_TEXT_POINT
            .add_row_offset(2)
            .composite("        [UpArrow]")
        + &HELP_TEXT_POINT.add_row_offset(3).composite("Left    [a]")
        + &HELP_TEXT_POINT
            .add_row_offset(4)
            .composite("        [LeftArrow]")
        + &HELP_TEXT_POINT.add_row_offset(5).composite("Right   [d]")
        + &HELP_TEXT_POINT
            .add_row_offset(6)
            .composite("        [RightArrow]")
        + &HELP_TEXT_POINT.add_row_offset(7).composite("Down    [s]")
        + &HELP_TEXT_POINT
            .add_row_offset(8)
            .composite("        [DownArrow]")
        + &HELP_TEXT_POINT.add_row_offset(9).composite("Reset   [r]")
        + &HELP_TEXT_POINT
            .add_row_offset(10)
            .composite("Drop    [space]")
        + &HELP_TEXT_POINT.add_row_offset(11).composite("Hold    [c]")
        + &HELP_TEXT_POINT
            .add_row_offset(12)
            .composite("Quit    [q]/[ESC]");
    #[cfg(feature = "with_log")]
    crate::logger::log_to_file(
        format_args!("help_info len = {}", help_info.as_bytes().len()),
        Some("string_lens"),
    );
    render(help_info)
}

#[inline(always)]
pub(crate) fn render<T: std::fmt::Display>(content: T) {
    term::stdout()
        .write_fmt(format_args!("{}", content))
        .unwrap()
}
