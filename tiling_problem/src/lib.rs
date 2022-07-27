#[cfg(test)]
mod tests {
    use crate::tiling_problem_solve;

    #[test]
    fn return_1_if_n_less_than_4() {
        assert_eq!(1, tiling_problem_solve(1));
        assert_eq!(1, tiling_problem_solve(2));
        assert_eq!(1, tiling_problem_solve(3));
    }

    #[test]
    fn return_2_if_n_is_4() {
        assert_eq!(2, tiling_problem_solve(4));
    }

    #[test]
    fn return_3_if_n_is_5() {
        assert_eq!(3, tiling_problem_solve(5))
    }

    #[test]
    fn return_4_if_n_is_8() {
        assert_eq!(7, tiling_problem_solve(8))
    }
}

//give a 4 x N rectangle
fn tiling_problem_solve(n: u8) -> u32 {
    if n < 4 {
        return 1;
    }

    if n == 4 {
        return 2;
    }

    tiling_problem_solve(n - 1) + tiling_problem_solve(n - 4)
}
