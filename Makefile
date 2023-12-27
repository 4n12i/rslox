check:
	cargo fmt
	cargo clippy -- -D warnings
	cargo test
	cargo check