/*
2563. Count the Number of Fair Pairs

Given a 0-indexed integer array nums of size n and two integers lower and upper, return the number of fair pairs.

A pair (i, j) is fair if:

0 <= i < j < n, and
lower <= nums[i] + nums[j] <= upper
 

Example 1:

Input: nums = [0,1,7,4,4,5], lower = 3, upper = 6
Output: 6
Explanation: There are 6 fair pairs: (0,3), (0,4), (0,5), (1,3), (1,4), and (1,5).
Example 2:

Input: nums = [1,7,9,2,5], lower = 11, upper = 11
Output: 1
Explanation: There is a single fair pair: (2,3).



*/

use std::cmp::Ordering;

impl Solution {
    // Function to count the number of "fair" pairs
    // A fair pair (i, j) satisfies: lower <= nums[i] + nums[j] <= upper
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut fair_pair_count = 0;
        
        // Sort the input vector nums
        nums.sort();
        
        // Helper function to perform binary search
        fn binary_search(arr: &[i32], target: i32) -> usize {
            let mut left = 0;
            let mut right = arr.len();
            
            while left < right {
                let mid = left + (right - left) / 2;
                if arr[mid] < target {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            left
        }

        // Iterate through each element in the sorted vector nums
        for i in 0..nums.len() {
            // We need to find the range of j such that:
            // lower <= nums[i] + nums[j] <= upper
            let lower_bound = lower - nums[i];
            let upper_bound = upper - nums[i];
            
            // Use binary search to find the indices of the valid range
            let lower_index = binary_search(&nums[i + 1..], lower_bound);
            let upper_index = binary_search(&nums[i + 1..], upper_bound + 1);

            // The number of valid pairs for the current nums[i]
            fair_pair_count += (upper_index - lower_index) as i64;
        }

        fair_pair_count
    }
}
