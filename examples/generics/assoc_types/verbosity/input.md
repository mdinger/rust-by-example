First, generic `trait`s on container types sometimes have too many type
specification requirements for users of the trait. This is generally a
problem when users don't require the generics it provides.

Specifically, examine the `Contains` `trait` and the `difference()` function
which utilizes it. The fact that `Contains` is generic immediately forces
users of the `trait` regardless of need to explicitly state *all* the
`trait`'s generic types.

{verbosity.play}

The problem is we require a way to express that `A` and `B` are determined
by the *input* `C`. Having to express them *input* parameters is just
hindering. Associated types provides exactly that capability.

### See also:

[`struct`s][structs], and [`trait`s][traits]


[structs]: /custom_types/structs.html
[traits]: /trait.html
