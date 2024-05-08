use std::fs::File;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use crate::util;

const KB: u64 = 1024;
const MB: u64 = 1024 * KB;

pub(crate) fn log_to_console<T>(msg: T)
where
    T: std::fmt::Debug,
{
    crate::term::clear_screen();
    crate::term::stdout()
        .write_fmt(format_args!("{:?}", msg))
        .unwrap();
}

#[allow(unused)]
pub(crate) fn log_to_file<T>(msg: T, specified_name: Option<&'static str>)
where
    T: std::fmt::Debug,
{
    static mut FILE: Option<File> = None;
    unsafe {
        let path_logfile = try_create_log_file(specified_name).expect("Create log dir failed!");
        let f = match FILE {
            Some(ref mut f) => {
                if f.metadata().unwrap().len() > 256 * KB {
                    FILE = Some(
                        OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .open(&path_logfile)
                            .expect("Create log file failed!"),
                    );
                    FILE.as_mut().unwrap()
                } else {
                    f
                }
            }
            None => {
                FILE = Some(
                    OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(&path_logfile)
                        .unwrap_or_else(|e| {
                            panic!(
                                "Error on log file create or open: path: {}\n Detail: {}",
                                path_logfile.to_str().unwrap(),
                                e
                            )
                        }),
                );
                FILE.as_mut().unwrap()
            }
        };
        f.write_fmt(format_args!("{:?}\n", msg)).unwrap();
        f.flush().unwrap()
    }
}

fn try_create_log_file(file_name: Option<&'static str>) -> Result<PathBuf, std::io::Error> {
    let mut path = util::bin_dir_path();
    path.push("log");
    if !path.exists() {
        fs::create_dir(&path)?;
    }
    path.push(format!(
        "{}.log",
        match file_name {
            Some(name) => name,
            _ => "tetris",
        }
    ));
    Ok(path)
}

#[cfg(test)]
mod logger_tests {
    use std::{
        env,
        path::{Path, PathBuf},
    };

    use crate::util;

    #[test]
    fn test_chrono_date() {
        let t = chrono::Local::now();
        println!("{}", t.to_string());
    }

    #[test]
    fn path_test() {
        let mut p = PathBuf::new();
        p.push("path1");
        assert_eq!(Path::new("path1"), &p);
        p.push("path2");
        assert_eq!(Path::new("path1\\path2"), &p);
    }

    #[test]
    fn program_dir_path_test() {
        let program_dir_path = util::bin_dir_path();
        assert!(program_dir_path.starts_with(r"C:\Users\feyn\source\rust_projects\tetris_rs"));
    }
}
