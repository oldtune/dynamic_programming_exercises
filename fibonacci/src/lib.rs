#[cfg(test)]
mod tests {
    use super::fibonacci;
    #[test]
    fn first_fibonacci_is_0() {
        assert!(0 == fibonacci(1));
    }

    #[test]
    fn second_fibonacci_is_1() {
        assert!(1 == fibonacci(2));
    }

    #[test]
    fn third_fibonacci_is_1() {
        assert!(1 == fibonacci(3));
    }

    #[test]
    fn fibonacci_should_return_correct_value() {
        assert!(55 == fibonacci(11));
    }
}

pub fn fibonacci(n: u64) -> u64 {
    if n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}
