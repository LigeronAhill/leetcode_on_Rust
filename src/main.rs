use crate::gcd_of_strings::gcd_of_strings;

mod gcd_of_strings;

fn main() {
    let test_data_1 = (String::from ("ABCABC"), String::from("ABC"), String::from("ABC"));
    if gcd_of_strings(test_data_1.0, test_data_1.1) == test_data_1.2 {
        println!("Test 1 - ok");
    } else {
        println!("Test 1 - failed");
    }
    let test_data_2 = (String::from("ABABAB"), String::from("ABAB"), String::from("AB"));
    if gcd_of_strings(test_data_2.0, test_data_2.1) == test_data_2.2 {
        println!("Test 2 - ok");
    } else {
        println!("Test 2 - failed");
    }
    let test_data_3 = (String::from("LEET"), String::from("CODE"), String::from(""));
    if gcd_of_strings(test_data_3.0, test_data_3.1) == test_data_3.2 {
        println!("Test 3 - ok");
    } else {
        println!("Test 3 - failed");
    }
}
