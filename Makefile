commit: 
	make check
	cargo test
	git add .
	git commit
	
run: 
	make check
	cargo test
	RUST_LOG=debug cargo run
	
build: 
	make check
	make test
	cargo build
	
test: 
	RUST_LOG=debug cargo test -- --nocapture

check:
	cargo fmt
	cargo clippy -- -D warnings
	cargo check