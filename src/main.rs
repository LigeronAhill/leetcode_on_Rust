use crate::kids_with_candies::kids_with_candies;

mod kids_with_candies;

fn main() {
    let test_data_1 = (vec![2, 3, 5, 1, 3], 3, vec![true, true, true, false, true]);
    let test_data_2 = (
        vec![4, 2, 1, 1, 2],
        1,
        vec![true, false, false, false, false],
    );
    let r1 = kids_with_candies(test_data_1.0, test_data_1.1);
    let r2 = kids_with_candies(test_data_2.0, test_data_2.1);
    if &r1 != &test_data_1.2 {
        println!("Test 1 failed with {:?}", r1);
    } else {
        println!("Test 1 passed");
    }
    if &r2 != &test_data_2.2 {
        println!("Test 2 failed with {:?}", r2);
    } else {
        println!("Test 2 passed");
    }
}
