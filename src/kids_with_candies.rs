
pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut result: Vec<bool> = Vec::new();
    for i in 0..candies.len() {
        result.push(candies[i] + extra_candies >= *candies.iter().max().unwrap());
    }
    result
}
