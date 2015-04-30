Rust provides a mechanism for spawning native OS threads via the `scoped`
function, the argument of this function is a moving closure.

These threads will be scheduled by the OS.
