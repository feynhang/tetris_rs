use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use rand::{rngs::ThreadRng, thread_rng, Rng};

pub fn timer(dur: Duration) -> bool {
    static mut START: Option<Instant> = None;
    unsafe {
        if START.is_none() {
            START = Some(Instant::now());
        }

        if START.as_ref().unwrap().elapsed() > dur {
            START.replace(Instant::now());
            return true;
        }
    }
    false
}

pub fn fps_count() -> u32 {
    static mut START: Option<Instant> = None;
    static mut FPS: u32 = 0;
    static mut FRAME_COUNT: u32 = 0;
    unsafe {
        if START.is_none() {
            START = Some(Instant::now());
        }

        FRAME_COUNT += 1;
        if START.as_ref().unwrap().elapsed() > Duration::from_secs(1) {
            START = Some(Instant::now());
            FPS = FRAME_COUNT;
            FRAME_COUNT = 0;
        }
        FPS
    }
}

// #[allow(unused)]
// pub fn slice_to_string<T>(slice: &[T]) -> String
// where
//     T: std::fmt::Debug,
// {
//     let mut s = "[".to_owned();
//     for t in slice {
//         s.push_str(&format!("\n{:?}", t));
//     }
//     s += "\n]";
//     s
// }

#[inline(always)]
pub fn rand_in_range(range: std::ops::Range<usize>) -> usize {
    thread_rng().gen_range(range)
}

#[cfg(windows)]
const SEP: char = '\\';
#[cfg(target_os = "linux")]
const SEP: char = '/';

pub fn bin_path_string() -> String {
    std::env::args().next().unwrap()
}

pub fn bin_dir_path() -> PathBuf {
    let exec_path_str = bin_path_string();
    let i = exec_path_str.rfind(SEP).unwrap();
    exec_path_str[0..i].parse().unwrap()
}

pub fn src_path() -> PathBuf {
    let mut project_dir = project_path();
    project_dir.push("src");
    project_dir
}

pub fn project_path() -> PathBuf {
    let exec_path_str = bin_path_string();
    exec_path_str[0..exec_path_str.find("target").unwrap()]
        .parse()
        .unwrap()
}
