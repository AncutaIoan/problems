/*
2434. Using a Robot to Print the Lexicographically Smallest String

You are given a string s and a robot that currently holds an empty string t. Apply one of the following operations until s and t are both empty:

Remove the first character of a string s and give it to the robot. The robot will append this character to the string t.
Remove the last character of a string t and give it to the robot. The robot will write this character on paper.
Return the lexicographically smallest string that can be written on the paper.

 

*/

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        use std::collections::VecDeque;

        let mut count = [0; 26];
        for &b in s.as_bytes() {
            count[(b - b'a') as usize] += 1;
        }

        let mut t = Vec::new(); // Stack
        let mut result = String::new();
        let mut s_iter = s.bytes();

        for c in s_iter {
            let idx = (c - b'a') as usize;
            count[idx] -= 1;
            t.push(c);

            // Find the smallest character remaining in s
            let mut min_char = 0;
            while min_char < 26 && count[min_char] == 0 {
                min_char += 1;
            }

            // Pop from t while it's safe
            while let Some(&last) = t.last() {
                if min_char == 26 || last <= (b'a' + min_char as u8) {
                    result.push(t.pop().unwrap() as char);
                } else {
                    break;
                }
            }
        }

        // Empty the rest of the stack
        while let Some(c) = t.pop() {
            result.push(c as char);
        }

        result
    }
}
