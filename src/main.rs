use crate::longest_consecutive::Solution;

mod longest_consecutive;

fn main() {
    let test_data_1 = (vec![100, 4, 200, 1, 3, 2], 4);
    let test_data_2 = (vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1], 9);
    let t1 = Solution::longest_consecutive(test_data_1.0);
    let t2 = Solution::longest_consecutive(test_data_2.0);
    if t1 != test_data_1.1 {
        println!("Test 1 failed");
    } else {
        println!("Test 1 passed");
    }
    if t2 != test_data_2.1 {
        println!("Test 2 failed");
    } else {
        println!("Test 2 passed");
    }
}
