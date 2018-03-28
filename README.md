# is_fourtytwo

A Rust library to check if a number is equal to fourtytwo.

## Usage

```rust
extern crate is_fourtytwo;
use is_fourtytwo::Is;

// simple
let a = Is::new(42).fourtytwo();
assert_eq!(true, a);

// advanced abstract numerical operations
let b = Is::new(42).plus(24).minus(24).fourtytwo();
assert_eq!(true, b);

// hard comparisons made easy
let c = Is::new(42).plus(1).greather_than();
assert_eq!(true, c);
```
## Acknowledgments

Freely inspired by [is-thirteen](https://github.com/jezen/is-thirteen) npm package.