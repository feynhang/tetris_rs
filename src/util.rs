use std::time::{Duration, Instant};

use rand::{thread_rng, Rng};

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


// #[cfg(feature = "with_log")]
// pub fn src_path() -> std::path::PathBuf {
//     let mut project_dir = project_path();
//     project_dir.push("src");
//     project_dir
// }

// #[cfg(feature = "with_log")]
// pub fn project_path() -> std::path::PathBuf {
//     let exe_dir = std::env::current_dir().unwrap();
//     let exe_dir_s = exe_dir.to_str().unwrap();
//     exe_dir_s[0..exe_dir_s.find("target").unwrap()]
//         .parse()
//         .unwrap()
// }
