/*
594. Longest Harmonious Subsequence

We define a harmonious array as an array where the difference between its maximum value and its minimum value is exactly 1.

Given an integer array nums, return the length of its longest harmonious subsequence among all its possible subsequences.


*/


use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut freq_map = HashMap::new();
        
        // Count occurrences of each number
        for &num in &nums {
            *freq_map.entry(num).or_insert(0) += 1;
        }
        
        let mut max_length = 0;
        
        // Check for harmonious subsequence
        for (&key, &value) in &freq_map {
            if let Some(&next_value) = freq_map.get(&(key + 1)) {
                max_length = max_length.max(value + next_value);
            }
        }
        
        max_length
    }
}
