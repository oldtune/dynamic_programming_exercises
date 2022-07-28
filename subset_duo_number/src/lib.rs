use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use crate::duo_number_solve;

    #[test]
    fn it_works() {
        let nums = vec![1, 2, 2, 3, 4, 5];
        duo_number_solve(0, &nums, vec![]);
    }
}

pub fn duo_number_solve(index: u8, nums: &[u8], subset: Vec<u8>) {
    if index == nums.len() as u8 {
        return;
    }

    // let mut hash_set = HashSet::new();

    if subset.len() < 2 || subset.contains(&nums[index as usize]) {
        let mut new_subset = subset.clone();
        new_subset.push(nums[index as usize]);

        duo_number_solve(index + 1, nums, new_subset);
    } else {
        println!("{:?}", subset);
    }

    duo_number_solve(index + 1, nums, vec![]);
}
