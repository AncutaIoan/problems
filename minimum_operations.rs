/*
3396. Minimum Number of Operations to Make Elements in Array Distinct

You are given an integer array nums. You need to ensure that the elements in the array are distinct. To achieve this, you can perform the following operation any number of times:

Remove 3 elements from the beginning of the array. If the array has fewer than 3 elements, remove all remaining elements.
Note that an empty array is considered to have distinct elements. Return the minimum number of operations needed to make the elements in the array distinct.

*/


use std::collections::HashSet;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        
        for ops in 0..=(n + 2) / 3 {
            let start = ops * 3;
            if start >= n {
                return ops as i32;
            }
            
            let mut seen = HashSet::new();
            let mut valid = true;

            for i in start..n {
                if seen.contains(&nums[i]) {
                    valid = false;
                    break;
                }
                seen.insert(nums[i]);
            }

            if valid {
                return ops as i32;
            }
        }

        ((n + 2) / 3) as i32
    }
}
