Associated types is an extension to generics which makes generics more
convenient, scalable, and powerful. We will examine a few problems with
generics and the solutions associated types provides. First, generic
container types sometimes suffer from overly heavy syntax.

Specifically, examine the `difference()` function in the next example
(also reproduced below). It is bounded by the `Contains` `trait`
forcing it to explicitly state *all* the `trait`'s generic types.

```rust
// `C` contains `A` and `B`. In light of that, having to express `A` and
// `B` again is a nuisance. Wouldn't it be convenient then to enable the
// `trait` to contain the types too so that `A` and `B` didn't need to
// be expressed here? Associated types allows exactly that.
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {
```

{assoc_types.play}

### See also:

[RFC](
https://github.com/aturon/rfcs/blob/associated-items/active/0000-associated-items.md
), [`struct`s][structs], and [`trait`s][traits]


[structs]: /custom_types/structs.html
[traits]: /trait.html
