Associated types (AT) is an extension to `trait` generics which makes generics
more convenient, scalable, and powerful. It does this by adding one new
concept:

* `trait`s can now internally define a type via the keyword `type`. This is
called an *output* type.

This results in some powerful consequences which will be discussed in the
following sections.

### See also:

[RFC](
https://github.com/aturon/rfcs/blob/associated-items/active/0000-associated-items.md
)
