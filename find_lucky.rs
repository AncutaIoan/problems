/*
1394. Find Lucky Integer in an Array

Given an array of integers arr, a lucky integer is an integer that has a frequency in the array equal to its value.

Return the largest lucky integer in the array. If there is no lucky integer return -1.
*/
use std::collections::HashMap;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut freq = HashMap::new();

        // Count the frequency of each number
        for &num in arr.iter() {
            *freq.entry(num).or_insert(0) += 1;
        }

        // Find the largest lucky number
        let mut result = -1;
        for (&num, &count) in freq.iter() {
            if num == count && num > result {
                result = num;
            }
        }

        result
    }
}
