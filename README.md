# microlisp

A minimal Lisp interpreter library written in Rust with no-std support.

## Features

- Core Lisp data types: numbers, booleans, symbols, lists, vectors
- Built-in functions and operators
- Variable bindings with lexical scoping
- `no_std` compatible (with `alloc`)

## Documentation

Detailed documentation, including a description of the instruction set can be
found in [doc/](doc/).

## Examples

You can find detailed examples in [examples](examples/). Run them with:
```rust
cargo run --example basic_usage
```

The test vectors in the codebase also make a great reference for how each
method should behave.

Below are some quick code snippets for illustrative purposes.

Parse and evaluate a simple expression:
```rust
let result = env.parse_eval("(+ 1 2 3)").unwrap();
println!("{}", result); // 6
```
Define variables:
```rust
env.parse_eval("(def x 42)").unwrap();
let result = env.parse_eval("(* x 2)").unwrap();
println!("{}", result); // 84
```
Local bindings with let:
```rust
let result = env.parse_eval("(let [a 10 b 20] (+ a b))").unwrap();
println!("{}", result); // 30
```
Conditional expressions:
```rust
let result = env.parse_eval("(if (> x 40) true false)").unwrap();
println!("{}", result); // true
```
Sequential execution with 'do':
```rust
let result = env.parse_eval("(do (def temp 5) (+ temp 10) (* temp 3))")?;
println!("(do (def temp 5) (+ temp 10) (* temp 3)) = {}", result);
```
Loops with dotimes:
```rust
env.parse_eval("(def sum 0)")?;
env.parse_eval("(dotimes [i 5] (def sum (+ sum i)))")?;
let result = env.parse_eval("(+ sum 0)")?; // Note: direct variable access needs expression context
println!("(dotimes [i 5] (def sum (+ sum i))) = {}", result);
```
