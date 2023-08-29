use crate::majority_element::majority_element;
mod majority_element;

fn main() {
    let test_data_1 = (vec![3,2,3], 3);
    let test_data_2 = (vec![2,2,1,1,1,2,2], 2);
    if majority_element(test_data_1.0) != test_data_1.1 {
        println!("Test 1 failed");
    } else {
        println!("Test 1 passed");
    }
    if majority_element(test_data_2.0) != test_data_2.1 {
        println!("Test 2 failed");
    } else {
        println!("Test 2 passed");
    }
}
