/*
1399. Count Largest Group

You are given an integer n.

Each number from 1 to n is grouped according to the sum of its digits.

Return the number of groups that have the largest size.

 

Example 1:

Input: n = 13
Output: 4
Explanation: There are 9 groups in total, they are grouped according sum of its digits of numbers from 1 to 13:
[1,10], [2,11], [3,12], [4,13], [5], [6], [7], [8], [9].
There are 4 groups with largest size.
Example 2:

Input: n = 2
Output: 2
Explanation: There are 2 groups [1], [2] of size 1.


*/

use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut sum_of_digits_map: HashMap<i32, i32> = HashMap::new();

        for i in 1..=n {
            let sum = sum_of_digits(i);
            *sum_of_digits_map.entry(sum).or_insert(0) += 1;
        }

        let max_count = sum_of_digits_map.values().cloned().max().unwrap_or(0);
        sum_of_digits_map.values().filter(|&&v| v == max_count).count() as i32
    }
}

fn sum_of_digits(n: i32) -> i32 {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .sum()
}
