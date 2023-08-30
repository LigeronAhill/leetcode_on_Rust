pub struct Solution;
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut result: Vec<char> = s.chars().collect();
        let mut left: usize = 0;
        let mut right = s.len();
        while left < right {
            if vowels.contains(&result[left]) {
                while left < right {
                    right -= 1;
                    if vowels.contains(&result[right]) {
                        result.swap(left, right);
                        break;
                    }
                }
            }
            left += 1;
        }

        result.into_iter().collect()
    }
}