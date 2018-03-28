# is-fourtytwo

A Rust library to check if a number is equal to fourtytwo.

## Usage

```rust
// simple
let a = Is::new(42).fourtytwo();
assert_eq!(true, a);

// advanced abstract numerical operations
let b = Is::new(42).plus(24).minus(24).fourtytwo();
assert_eq!(true, b);
```
## Acknowledgments

Freely inspired by [is-thirteen](https://github.com/jezen/is-thirteen) npm package.