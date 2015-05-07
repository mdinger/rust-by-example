When working with generics, the types often must use traits as *bounds* to
stipulate what functionality a type implements. For example, the following
example uses the trait `Display` to print and so it requires `T` to be bound
by `Display`; that is, `T` *must* implement `Display`.

```rust
// This function reads: `printer` takes generic type `T` which
// must implement trait `Display`.
fn printer<T: Display>(t: T) {
    println!("{}", t);
}
```

Bounding allows generic instances to access the methods of the traits
specified in the bounds. For example:

{bounds.play}

Bounding also restricts the generic from being specialized to
types that do *not* conform to the bounds. That is:

```rust
struct S<T: Display>(T);

// Error! `Vec<T>` does not implement `Display`. This
// specialization will fail.
let s = S(vec![1]);
```

