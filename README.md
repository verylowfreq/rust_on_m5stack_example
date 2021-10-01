# Rust on M5Stack

The Rust example for M5Stack that can draw LCD and treat BtnA, BtnB, and BtnC.

Based on [Rust on ESP32 by Kerkour](https://kerkour.com/blog/rust-on-esp32)


RustでM5Stackのプログラムを書くサンプルプロジェクトです。ビルド環境はDockerで準備します。
VSCodeでの編集やビルドを想定。

### Build with VSCode

Install VSCode extensions: Remote Container, Rust, and rust-analyer.
Open the project folder with VSCode and "Reopen in container".
`cd rust_on_m5stack` and `cargo build` (NOTE: execute inside of the docker container)
