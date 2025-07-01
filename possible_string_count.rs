/*
3330. Find the Original Typed String I

Alice is attempting to type a specific string on her computer. However, she tends to be clumsy and may press a key for too long, resulting in a character being typed multiple times.

Although Alice tried to focus on her typing, she is aware that she may still have done this at most once.

You are given a string word, which represents the final output displayed on Alice's screen.

Return the total number of possible original strings that Alice might have intended to type.
*/
impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let chars: Vec<char> = word.chars().collect();
        let mut total = 1; // always count the original word
        let mut i = 0;
        let n = chars.len();

        while i < n {
            let mut count = 1;
            while i + 1 < n && chars[i] == chars[i + 1] {
                count += 1;
                i += 1;
            }
            total += count - 1;
            i += 1;
        }

        total
    }
}
