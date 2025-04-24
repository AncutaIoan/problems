/*
2799. Count Complete Subarrays in an Array

You are given an array nums consisting of positive integers.

We call a subarray of an array complete if the following condition is satisfied:

The number of distinct elements in the subarray is equal to the number of distinct elements in the whole array.
Return the number of complete subarrays.

A subarray is a contiguous non-empty part of an array.

 

Example 1:

Input: nums = [1,3,1,2,2]
Output: 4
Explanation: The complete subarrays are the following: [1,3,1,2], [1,3,1,2,2], [3,1,2] and [3,1,2,2].
Example 2:

Input: nums = [5,5,5,5]
Output: 10
Explanation: The array consists only of the integer 5, so any subarray is complete. The number of subarrays that we can choose is 10.
 

Constraints:

1 <= nums.length <= 1000
1 <= nums[i] <= 2000
*/
// use std::collections::HashSet;

// impl Solution {
//     pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
//         let n = nums.len();
//         let total_distinct: usize = nums.iter().collect::<HashSet<_>>().len();
//         let mut count = 0;

//         for i in 0..n {
//             let mut seen = HashSet::new();
//             for j in i..n {
//                 seen.insert(nums[j]);
//                 if seen.len() == total_distinct {
//                     count += 1;
//                 }
//             }
//         }

//         count
//     }
// }

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let total_distinct = nums.iter().collect::<HashSet<_>>().len();
        let mut result = 0;
        let mut left = 0;
        let mut freq = HashMap::new();

        for right in 0..n {
            *freq.entry(nums[right]).or_insert(0) += 1;

            while freq.len() == total_distinct {
                result += (n - right) as i32;

                if let Some(count) = freq.get_mut(&nums[left]) {
                    *count -= 1;
                    if *count == 0 {
                        freq.remove(&nums[left]);
                    }
                }
                left += 1;
            }
        }

        result
    }
}
