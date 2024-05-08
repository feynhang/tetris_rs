// use std::mem::size_of_val;

fn main() {
    tetris_rs::Tetris::instance()
        .set_border_style(tetris_rs::style::BorderStyle::DoubleLine)
        .start();
}
