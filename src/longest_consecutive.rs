pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut temp = nums;
        temp.sort();
        let mut count = 1;
        let mut max_count = 1;
        for i in 0..temp.len() - 1 {
            if temp[i] == temp[i + 1] {
                continue;
            }
            if temp[i] + 1 == temp[i + 1] {
                count += 1;
                if count > max_count {
                    max_count = count;
                }
            } else {
                count = 1;
            }
        }
        max_count
    }
}
