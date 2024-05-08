use std::{
    fmt::Display,
    io::{Stdout, Write},
    sync::OnceLock,
};

pub(crate) const CSI: &str = "\u{1b}[";
const RESET_COLOR: &str = "\u{1b}[0m";

pub(crate) fn stdout() -> &'static mut std::io::BufWriter<Stdout> {
    static mut BUF_OUT: OnceLock<std::io::BufWriter<Stdout>> = OnceLock::new();
    unsafe {
        match BUF_OUT.get_mut() {
            Some(out) => out,
            None => {
                BUF_OUT
                    .set(std::io::BufWriter::with_capacity(128, std::io::stdout()))
                    .unwrap();
                BUF_OUT.get_mut().unwrap()
            }
        }
    }
}

#[inline(always)]
pub(crate) fn clear_screen() {
    stdout().write_fmt(format_args!("{}2J", CSI)).unwrap()
}
#[inline(always)]

pub(crate) fn show_cursor() {
    stdout().write_fmt(format_args!("{}?25h", CSI)).unwrap()
}

#[inline(always)]
pub(crate) fn hide_cursor() {
    stdout().write_fmt(format_args!("{}?25l", CSI)).unwrap()
}

pub(crate) trait TermColorful {
    fn with_bg(&self, color_id: u8) -> String;
    fn with_fg(&self, color_id: u8) -> String;
}

impl<T> TermColorful for T
where
    T: Display,
{
    fn with_bg(&self, color_id: u8) -> String {
        format!("{}48;5;{}m{}{}", CSI, color_id, self, RESET_COLOR)
    }

    fn with_fg(&self, color_id: u8) -> String {
        format!("{}38;5;{}m{}{}", CSI, color_id, self, RESET_COLOR)
    }

}
