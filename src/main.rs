use crate::reverse_vowels::Solution;

mod reverse_vowels;

fn main() {
    struct TestData {
        input: String,
        output: String,
    }
    let test_1: TestData = TestData {
        input: String::from("leetcode"),
        output: String::from("leotcede"),
    };
    let test_2: TestData = TestData {
        input: String::from("hello"),
        output: String::from("holle"),
    };
    if Solution::reverse_vowels(test_1.input) == test_1.output {
        println!("Test 1 passed");
    } else {
        println!("Test 1 failed");
    }
    if Solution::reverse_vowels(test_2.input) == test_2.output {
        println!("Test 2 passed");
    } else {
        println!("Test 2 failed");
    }
}
