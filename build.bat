@REM https://gnutoolchains.com/raspberry/
rustup target add armv7-unknown-linux-gnueabihf
cargo build --release --target=armv7-unknown-linux-gnueabihf
