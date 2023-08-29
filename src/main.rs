use crate::can_place_flowers::can_place_flowers;

mod can_place_flowers;

fn main() {
    let test_data_1 = (vec![1, 0, 0, 0, 1], 1, true);
    let test_data_2 = (vec![1, 0, 0, 0, 1], 2, false);
    if can_place_flowers(test_data_1.0, test_data_1.1) != test_data_1.2 {
        println!("Test 1 failed");
    } else {
        println!("Test 1 passed");
    }
    if can_place_flowers(test_data_2.0, test_data_2.1) != test_data_2.2 {
        println!("Test 2 failed");
    } else {
        println!("Test 2 passed");
    }
}
