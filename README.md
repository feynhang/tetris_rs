# Tetris game on console, implemented using rust

## Build

You need to have [rust](https://www.rust-lang.org) installed on the build machine (which should also be the target platform). Then cd to `tetris_rs` ( ***PROJECT ROOT DIRECTORY*** ) and run it in shell:

```shell
cargo build --release
```

Now, the binary file should be in directory: `tetris_rs/target/release`, its file name without extension should be **tetris_rs**.

## Important

In this implementation, the number of columns occupied by each mino is actually two columns in the console.
