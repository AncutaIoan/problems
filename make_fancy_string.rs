/*
1957. Delete Characters to Make Fancy String

A fancy string is a string where no three consecutive characters are equal.

Given a string s, delete the minimum possible number of characters from s to make it fancy.

Return the final string after the deletion. It can be shown that the answer will always be unique.
*/
impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut result = String::new();
        let mut prev_char = '\0'; // placeholder for previous char
        let mut count = 0;

        for c in s.chars() {
            if c == prev_char {
                count += 1;
            } else {
                count = 1;
                prev_char = c;
            }

            if count < 3 {
                result.push(c);
            }
        }

        result
    }
}
