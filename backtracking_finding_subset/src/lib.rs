#[cfg(test)]
mod tests {
    use crate::finding_subset_solve;

    #[test]
    fn it_works() {
        let set = vec![1, 2, 3];
        finding_subset_solve(0, &set, vec![]);
    }
}

pub fn finding_subset_solve(depth: u8, set: &Vec<u8>, subset: Vec<u8>) {
    if depth == set.len() as u8 {
        println!("{:?}", &subset);
        return;
    }
    let mut new_set = subset.clone();
    new_set.push(set[depth as usize]);

    finding_subset_solve(depth + 1, &set, new_set);
    finding_subset_solve(depth + 1, &set, subset.clone());
}
