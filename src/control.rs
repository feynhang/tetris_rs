use crate::Tetris;

const KEY_Q: char = 'q';
const KEY_W: char = 'w';
const KEY_A: char = 'a';
const KEY_S: char = 's';
const KEY_D: char = 'd';
const KEY_Z: char = 'z';
const KEY_X: char = 'x';
const KEY_C: char = 'c';
const KEY_R: char = 'r';
const KEY_H: char = 'h';

const KEY_SPACE: char = ' ';

pub fn map_action(ch: char) {
    match ch {
        KEY_A => left(),
        KEY_D => right(),
        KEY_S => down(),
        KEY_W => rotate_r(),
        KEY_Z => rotate_l(),
        KEY_X => rotate_2(),
        KEY_C => hold(),
        KEY_SPACE => drop(),
        KEY_Q => quit(),
        KEY_R => reset(),
        KEY_H => help(),
        _ => (),
    };
}

pub(crate) fn readkey_event() {
    while Tetris::instance().running() {
        let c = match console::Term::stdout().read_key().unwrap() {
            console::Key::Char(ch) => {
                if !ch.is_ascii() {
                    continue;
                }
                if ch.is_ascii_lowercase() {
                    ch
                } else {
                    ch.to_ascii_lowercase()
                }
            }
            console::Key::ArrowLeft => KEY_A,
            console::Key::ArrowRight => KEY_D,
            console::Key::ArrowDown => KEY_S,
            console::Key::ArrowUp => KEY_W,
            console::Key::Escape => KEY_Q,
            console::Key::CtrlC => {
                eprintln!("Force exit tetris game!");
                std::process::exit(1);
            }
            _ => continue,
        };
        map_action(c);
    }
}

fn drop() {
    Tetris::instance().drop()
}

fn left() {
    Tetris::instance().left();
}

fn right() {
    Tetris::instance().right();
}

fn down() {
    Tetris::instance().down();
}

fn rotate_r() {
    Tetris::instance().rotate(crate::base::tetromino::TetroState::Right);
}

fn rotate_l() {
    Tetris::instance().rotate(crate::base::tetromino::TetroState::Left);
}

fn rotate_2() {
    Tetris::instance().rotate(crate::base::tetromino::TetroState::Second);
}

fn quit() {
    Tetris::instance().set_running(false)
}

fn hold() {
    Tetris::instance().exchange_hold();
}

fn reset() {
    Tetris::instance().reset();
}

fn help() {
    Tetris::instance().help();
}
