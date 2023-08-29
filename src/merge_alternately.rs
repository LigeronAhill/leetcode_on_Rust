pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut first_chars = word1.chars();
        let mut second_chars = word2.chars();
        let mut result = Vec::with_capacity(word1.len() + word2.len());
        let min_len = word1.len().min(word2.len());
        for _ in 0..min_len {
            result.push(first_chars.next().unwrap());
            result.push(second_chars.next().unwrap());
        }
        result.extend(first_chars);
        result.extend(second_chars);
        let result_string: String = result.into_iter().collect();
        result_string
    }