# rslox
Rust implementation of a Lox interpreter  

## Usage
```sh
# Run prompt
$ cargo run 

# Run a lox file
$ cargo run <file>
```

With debug log: 
```sh 
$ RUST_LOG=debug cargo run
```

## Implementation
- [x] Scanning 
- [x] Representing Code
- [x] Parsing Expressions
- [x] Evaluating Expressions
- [x] Statements and State
- [x] Control Flow
- [ ] Functions
- [ ] Resolving and Binding
- [ ] Classes
- [ ] Inheritance

## Reference
[Crafting Interpreters](https://github.com/munificent/craftinginterpreters)

## License
[MIT](./LICENSE)