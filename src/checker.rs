trait ResultChecker {
    fn is_result() -> bool;
}

impl<T> ResultChecker for &T {
    fn is_result() -> bool { false }
}

impl<T, E> ResultChecker for Result<T, E> {
    fn is_result() -> bool { true }
}
