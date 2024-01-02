commit: 
	make check
	git add .
	git commit
	
build: 
	make check
	cargo build

check:
	cargo fmt
	cargo clippy -- -D warnings
	cargo test -- --nocapture
	cargo check