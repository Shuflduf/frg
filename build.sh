RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none"
cargo build --release
upx --best --lzma target/release/frg
