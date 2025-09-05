RUSTFLAGS="-C target-feature=+crt-static" cargo build --release
sudo cp ./target/release/asi /run/media/$(whoami)/Ventoy/
