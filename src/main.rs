// use std::mem::size_of_val;


fn main() {
    // let (cols, rows) = console::Term::buffered_stdout().size();
    // println!("cols = {}, rows = {}", rows, cols);
    // let s = "\u{25e3}\u{25e5}";
    // println!("{}", size_of_val(s));
    tetris_rs::Tetris::instance().set_border_style(tetris_rs::style::BorderStyle::DoubleLine).start();
    // println!("FRAME_BLANK size = {}", tetris_rs::const_vals::FRAME_BLANK.len())
}
