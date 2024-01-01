build: 
	make check
	cargo build
	
test: 
	make check
	cargo test -- --nocapture

commit: 
	make check
	git add .
	git commit
	
check:
	cargo fmt
	cargo clippy -- -D warnings
	cargo test
	cargo check