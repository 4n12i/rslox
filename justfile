# default recipe. Formatter, lint and test
check: 
    cargo fmt 
    cargo clippy -- -D warnings 
    cargo check
    cargo test

# run a lox file. If omitted, launches prompt
run FILE='': 
    just check
    cargo run {{FILE}}

# run a lox file with debug log. If omitted, launches prompt
debug FILE='': 
    just check 
    RUST_LOG=debug cargo run {{FILE}}
    
# create a commit
commit: 
    just check 
    git add .
    git commit
