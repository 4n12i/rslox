commit: 
	make check
	git add .
	git commit
	
run: 
	make check
	RUST_LOG=debug cargo run
	
build: 
	make check
	cargo build

check:
	cargo fmt
	cargo clippy -- -D warnings
	cargo test -- --nocapture
	cargo check