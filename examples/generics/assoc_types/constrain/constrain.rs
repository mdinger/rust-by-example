use std::fmt::Display;

#[derive(Copy, Clone)]
struct Container<T>(T);

trait Contains {
    type A;

    fn inner(&self) -> Self::A;
}

impl<T: Copy> Contains for Container<T> {
    type A = T;

    fn inner(&self) -> Self::A {
        let Container(i) = *self;

        i
    }
}

fn printer_longform<C>(c: C) where
    C: Contains,
    C::A: Display {
    println!("{}", c.inner());
}

fn num<C>(c: C) -> i32 where
// Shorthand for this (which isn't supported yet anyway):
// C: Contains,
// C::A = i32
    C: Contains<A = i32> {
    c.inner()
}
/*
fn printer_compact<C>(c: C) where
    C: Contains<A: Display> {
    println!("{}", c.inner());
}*/

fn main() {
    let i = Container(32);
    let f = Container(64.0);
    
    printer_longform(i);
    //printer_compact(f);
    
    println!("{}", num(i));
}
