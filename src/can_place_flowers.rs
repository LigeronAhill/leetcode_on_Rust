pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut count = 0;
    let mut temp = flowerbed;
    for i in 0..temp.len() {
        if temp[i] == 0 {
            let left = { i == 0 || temp[i - 1] == 0 };
            let right = { i == temp.len() - 1 || temp[i + 1] == 0 };
            if left && right {
                temp[i] = 1;
                count += 1;
            }
        }
    }
    count >= n
}
