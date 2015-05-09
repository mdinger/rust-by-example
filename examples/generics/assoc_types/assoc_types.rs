// Define 2 number types and store them in a container structure.
// Derive `Clone` for `.clone()` and `PartialEq` for the `==`
// operator.
#[derive(Clone, PartialEq)] struct Number(i32);
#[derive(Clone, PartialEq)] struct Digit(i32);
struct Container(Number, Digit);

// Define a trait generic over 2 types. Further methods besides
// the first test do not depend on the generic types.
trait Contains<A, B> {
    fn contains(&self, &A, &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<Number, Digit> for Container {
    // True if the numbers stored are equal.
    fn contains(&self, number: &Number, digit: &Digit) -> bool {
        let Container(ref n, ref d) = *self;

        (n == number) && (d == digit)
    }
    // Grab the first number.
    fn first(&self) -> i32 {
        let Container(Number(ref n), _) = *self;

        n.clone()
    }
    // Grab the last number.
    fn last(&self) -> i32 {
        let Container(_, Digit(ref d)) = *self;

        d.clone()
    }
}

// Has generic type parameters `A` and `B` in spite of only using `C`.
// This would be much clearer if it only depended on `C`.
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {
    container.first() - container.last()
}

fn main() {
    let n = Number(10);
    let d = Digit(3);

    let container = Container(n.clone(), d.clone());

    println!("Does Container contain 10 and 3: {}",
        container.contains(&n, &d));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    
    println!("The difference is: {}", difference(&container));
}
