pub fn cal_factorial(n: u64) -> u64 {
    if n == 1 {
        return 1;
    }

    n * cal_factorial(n - 1)
}
