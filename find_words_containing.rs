/*
2942. Find Words Containing Character

You are given a 0-indexed array of strings words and a character x.

Return an array of indices representing the words that contain the character x.

Note that the returned array may be in any order.

*/

impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut result = Vec::new();
        for (i, word) in words.iter().enumerate() {
            if word.contains(x) {
                result.push(i as i32);
            }
        }
        result
    }
}
