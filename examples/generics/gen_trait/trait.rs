// A non-copyable type.
struct Empty;

// A trait generic over `T`.
trait Die<T> {
    // It defines a method on the caller type.
    fn die(self);
}

// Implement `Die<T>` for any generic `T`.
impl<T> Die<T> for T {
    // This method takes ownership of the caller and then exits;
    // essentially deallocating the caller.
    fn die(self) {}
}

fn main() {
    let empty = Empty;

    // Deallocate empty.
    empty.die();

    //empty;
    // ^ TODO: Try uncommenting this line.
}
