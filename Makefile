build: 
	make check
	cargo build

commit: 
	make check
	git add .
	git commit
	
check:
	cargo fmt
	cargo clippy -- -D warnings
	cargo test
	cargo check