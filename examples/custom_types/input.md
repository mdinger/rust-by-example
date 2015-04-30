Rust custom data types are formed mainly through the two keywords:

* `struct`: define a structure
* `enum`: define an enumeration

```rust
struct S {
    f: fn(Vec<i32>) -> i32,
}

fn main() {
    let s = S{
        f: super_magic,
    };
}

fn super_magic(_v: Vec<i32>) -> i32 {
    1
}
```
