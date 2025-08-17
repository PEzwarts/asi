RUSTFLAGS="-C target-feature=+crt-static" cargo build --release
sudo cp ./target/release/arch /run/media/$(whoami)/Ventoy/
