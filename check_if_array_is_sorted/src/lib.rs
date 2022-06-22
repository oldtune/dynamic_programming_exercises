#[cfg(test)]
mod tests {
    use crate::check_if_array_is_sorted;

    #[test]
    fn return_true_if_len_is_0_or_1() {
        assert_eq!(true, check_if_array_is_sorted(vec![]));
        assert_eq!(true, check_if_array_is_sorted(vec![1]));
    }

    #[test]
    fn return_true_if_array_is_sorted() {
        assert_eq!(true, check_if_array_is_sorted(vec![1, 2]));
    }

    #[test]
    fn return_true_if_long_array_is_sorted() {
        assert_eq!(
            true,
            check_if_array_is_sorted(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
        );
    }

    #[test]
    fn return_false_if_not_sorted() {
        assert_eq!(false, check_if_array_is_sorted(vec![1, 3, 2, 4, 5, 6, 5]));
    }
}

fn check_if_array_is_sorted(a: Vec<u8>) -> bool {
    let len = a.len();
    if len == 0 || len == 1 {
        return true;
    }

    if len == 2 {
        return a[0] <= a[1];
    }

    return check_if_array_is_sorted(a[1..len].into());
}
