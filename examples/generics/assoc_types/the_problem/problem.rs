struct Container(i32, i32);

// A trait which will check to see if 2 items are stored inside of container.
// Also retrieves first or last value.
trait Contains<A, B> {
    fn contains(&self, &A, &B) -> bool; // Explicitly requires `A` and `B`.
    fn first(&self) -> i32; // Doesn't explicitly require `A` or `B`.
    fn last(&self) -> i32;  // Doesn't explicitly require `A` or `B`.
}

impl Contains<i32, i32> for Container {
    // True if the numbers stored are equal.
    fn contains(&self, number: &i32, digit: &i32) -> bool {
        let Container(ref n, ref d) = *self;

        (n == number) && (d == digit)
    }
    // Grab the first number.
    fn first(&self) -> i32 {
        let Container(n, _) = *self;

        n
    }
    // Grab the last number.
    fn last(&self) -> i32 {
        let Container(_, d) = *self;

        d
    }
}

// `C` contains `A` and `B`. In light of that, having to express `A` and
// `B` again is a nuisance.
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {
    container.last() - container.first()
}

fn main() {
    let number = 3;
    let digit = 10;

    // `container` will be the `input` type. When a method is called
    // on `container`, `A` and `B` will be determined immediately
    // because they're internal to `container`.
    let container = Container(number, digit);

    println!("Does container contain {} and {}: {}",
        &number, &digit,
        container.contains(&number, &digit));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
