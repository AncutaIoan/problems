/*
2566. Maximum Difference by Remapping a Digit

You are given an integer num. You know that Bob will sneakily remap one of the 10 possible digits (0 to 9) to another digit.

Return the difference between the maximum and minimum values Bob can make by remapping exactly one digit in num.

Notes:

When Bob remaps a digit d1 to another digit d2, Bob replaces all occurrences of d1 in num with d2.
Bob can remap a digit to itself, in which case num does not change.
Bob can remap different digits for obtaining minimum and maximum values respectively.
The resulting number after remapping can contain leading zeroes.

*/

impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let num_str = num.to_string();
        
        // Compute the maximum possible value
        let mut max_val = num;
        for ch in num_str.chars() {
            if ch != '9' {
                let replaced = num_str.replace(ch, "9");
                let replaced_num = replaced.parse::<i32>().unwrap();
                if replaced_num > max_val {
                    max_val = replaced_num;
                }
            }
        }

        // Compute the minimum possible value
        let mut min_val = num;
        for ch in num_str.chars() {
            if ch != '0' {
                let replaced = num_str.replace(ch, "0");
                let replaced_num = replaced.parse::<i32>().unwrap();
                if replaced_num < min_val {
                    min_val = replaced_num;
                }
            }
        }

        max_val - min_val
    }
}
