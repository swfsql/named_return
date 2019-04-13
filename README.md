# Named Return

Declares a proc-macro that enables return types to be named.  
Mostly re-defining structures from `syn` for parsing.

The macro also:
1. Declares the variables that were named as a prefix statement to the
original function's body.
2. Requires that the return syntax is similar to the input parameter
syntax.
    - Requires parentheses.

## Example

```rust
#![feature(proc_macro_hygiene)]
use named_return::named_return;
#
# #[derive(Debug, PartialEq, Eq)]
# pub struct A;
# #[derive(Debug, PartialEq, Eq)]
# pub struct B;

named_return!(
fn f() -> (a: A, b: B) {
    a = A;
    b = B;
    (a, b)
});

assert_eq!(f(), (A, B));
```

## Note

The intended syntax were to be used with a proc-macro-attr, such as:

```rust,ignore
#[named_return]
fn f() -> (a: A, b: B) {
    a = A;
    b = B;
    (a, b)
}
```

But it seems that Rust parses the original function syntax before
executing the proc-macro-attr and so it refuses the invalid syntax.

This is a draft and is based on this suggestion:
https://github.com/rust-lang/rfcs/issues/2638