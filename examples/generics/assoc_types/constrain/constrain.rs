use std::fmt::Display;

struct Container<T>(T);

trait Contains {
    type A;

    fn inner(&self) -> Self::A;
}

impl<T: Clone> Contains for Container<T> {
    type A = T;

    fn inner(&self) -> Self::A {
        let Container(ref i) = *self;

        i.clone()
    }
}

// Bounds on associated types.
fn printer<C>(c: C) where
    C: Contains,
    C::A: Display {
    println!("{}", c.inner());
}

// The assignment shorthand:
fn num_small<C>(c: C) -> i32 where
    C: Contains<A = i32> {
    c.inner()
}


fn main() {
    printer(Container(17i32));

    println!("{}", num_small(Container(4i32)));
}
