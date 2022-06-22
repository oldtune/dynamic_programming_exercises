extern crate factorial;
#[cfg(test)]
mod tests {
    use factorial::cal_factorial;

    #[test]
    fn factorial_of_one_should_return_1() {
        assert!(1 == cal_factorial(1));
    }

    #[test]
    fn factorial_should_return_correct_answer() {
        assert!(6 == cal_factorial(3));
    }
}
