Implementing `fmt::Display` for a structure where the elements must each be
handled individually is tricky. The problem is that each `write!` generates a
`fmt::Result`. Proper handling of this requires dealing with *all* the
results. Rust provides the `try!` macro for exactly this purpose.

Using `try!` on `write!` looks like this:

```rust
// Try `write!` to see if it errors. If it errors, return
// the error. Otherwise continue.
try!(write!(f, "{}", value));
```

With `try!` available, implementing `fmt::Display` for a `Vec` is
straightforward:

{testcase_list.play}

### See also:

[`for`][for], [`ref`][ref], [`Result`][result], [`struct`][struct],
[`try!`][try], and [`vec!`][vec]

[for]: /flow_control/for.html
[result]: /std/result.html
[ref]: /scope/borrow/ref.html
[struct]: /custom_types/structs.html
[try]: /std/result/try.html
[vec]: /std/vec.html
