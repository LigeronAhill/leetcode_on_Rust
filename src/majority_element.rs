use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut m = HashMap::new();
    let l = nums.len();
    let majority_count = l / 2;

    for num in nums {
        let count = m.entry(num).or_insert(0);
        *count += 1;
        if *count > majority_count {
            return num;
        }
    }
    0
}
