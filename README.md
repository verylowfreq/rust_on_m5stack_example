# Rust on M5Stack

The Rust example for M5Stack that can draw LCD and treat BtnA, BtnB, and BtnC.

Based on [Rust on ESP32 by Kerkour](https://kerkour.com/blog/rust-on-esp32)


RustでM5Stackのプログラムを書くサンプルプロジェクトです。ビルド環境はDockerで準備します。
VSCodeでの編集やビルドを想定。

### Build with VSCode

 1) Install VSCode extensions: Remote Container, and rust-analyer.
 1) Install tools: `cargo install espflash espmonitor`
 1) Open the project folder with VSCode and "Reopen in container".
 1) Run `cargo +esp build` inside of the docker container.
 1) `espflash COM3 target/xtensa-esp32-espidf/debug/rust_on_m5stack` (If M5Stack is COM3, at host's terminal)
 1) `espmonitor COM3`
