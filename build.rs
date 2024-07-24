#[cfg(feature = "test_wallkick")]
fn dest_map_path() -> std::path::PathBuf {
    std::path::PathBuf::from("target/debug/tetris.map")
}


#[cfg(feature = "test_wallkick")]
fn copy_map_file() {
    let src_map = std::path::Path::new("src/tetris.map");
    let dest_map = dest_map_path();
    _ = std::fs::copy(src_map, dest_map).expect("Failed to copy map file!")
}

fn main() {
    #[cfg(feature = "test_wallkick")]
    copy_map_file();
}
