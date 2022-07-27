#[cfg(test)]
mod backtracking_permutation_test {
    #[test]
    fn permutation_works() {
        crate::permutation();
    }
}

fn permutation() {
    let numbers = vec![1, 2, 3, 4];
    let mut track = vec![];
    sub_permutation(&numbers, &mut track);
}

fn sub_permutation(numbers: &Vec<u8>, track: &mut Vec<u8>) {
    if numbers.len() == track.len() {
        println!("{:?}", track);
        return;
    }

    for &number in numbers {
        if track.contains(&number) {
            continue;
        }

        track.push(number);

        sub_permutation(numbers, track);

        track.pop();
    }
}
