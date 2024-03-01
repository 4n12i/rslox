# rslox
Rust implementation of an interpreter for the Lox programming language  

## Usage
Run prompt: 

```bash
cargo run 
```

Run a lox file: 

```bash
cargo run  -- examples/hello.lox
```

With debug log: 

```bash
RUST_LOG=debug cargo run
```

## Implementation
- [x] Scanning 
- [x] Representing Code
- [x] Parsing Expressions
- [x] Evaluating Expressions
- [x] Statements and State
- [x] Control Flow
- [x] Functions
- [ ] Resolving and Binding
- [ ] Classes
- [ ] Inheritance

## Reference
[Crafting Interpreters](https://github.com/munificent/craftinginterpreters)

## License
[MIT](./LICENSE)
