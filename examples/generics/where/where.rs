use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(&self);
}

impl<T> PrintInOption for T where
    // Required to access `.clone()`.
    T: Clone,
    // Without a `where` clause we would have to express this as
    // `T: Debug` or use some other method to indirectly approach
    // this. The bound we want though is `Option<T>: Debug` because
    // that's what being printed. To do otherwise would be to use
    // the wrong bound. This requires a `where` clause.
    Option<T>: Debug {
    fn print_in_option(&self) {
        // Clone to prevent a move into the option.
        let x = self.clone();
        println!("{:?}", Some(x));
    }
}

fn main() {
    let i = 12;

    i.print_in_option();
}
