/*
1717. Maximum Score From Removing Substrings

You are given a string s and two integers x and y. You can perform two types of operations any number of times.

Remove substring "ab" and gain x points.
For example, when removing "ab" from "cabxbae" it becomes "cxbae".
Remove substring "ba" and gain y points.
For example, when removing "ba" from "cabxbae" it becomes "cabxe".
Return the maximum points you can gain after applying the above operations on s.
*/

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        // Helper function to remove all "ab" or "ba" pairs
        fn remove_pair(s: Vec<char>, first: char, second: char, score: i32) -> (Vec<char>, i32) {
            let mut stack = Vec::new();
            let mut total = 0;

            for c in s {
                if let Some(&last) = stack.last() {
                    if last == first && c == second {
                        stack.pop();
                        total += score;
                    } else {
                        stack.push(c);
                    }
                } else {
                    stack.push(c);
                }
            }

            (stack, total)
        }

        let chars: Vec<char> = s.chars().collect();
        let (remaining, mut total) = if x > y {
            // First remove all "ab" for higher score
            let (rem, gain) = remove_pair(chars, 'a', 'b', x);
            (rem, gain)
        } else {
            // First remove all "ba" for higher score
            let (rem, gain) = remove_pair(chars, 'b', 'a', y);
            (rem, gain)
        };

        // Second remove the lower score pair
        let (final_rem, gain2) = if x > y {
            remove_pair(remaining, 'b', 'a', y)
        } else {
            remove_pair(remaining, 'a', 'b', x)
        };

        total += gain2;
        total
    }
}
