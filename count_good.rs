/*


2537. Count the Number of Good Subarrays

Given an integer array nums and an integer k, return the number of good subarrays of nums.

A subarray arr is good if there are at least k pairs of indices (i, j) such that i < j and arr[i] == arr[j].

A subarray is a contiguous non-empty sequence of elements within an array.

 

Example 1:

Input: nums = [1,1,1,1,1], k = 10
Output: 1
Explanation: The only good subarray is the array nums itself.
Example 2:

Input: nums = [3,1,4,3,2,2,4], k = 2
Output: 4
Explanation: There are 4 different good subarrays:
- [3,1,4,3,2,2] that has 2 pairs.
- [3,1,4,3,2,2,4] that has 3 pairs.
- [1,4,3,2,2,4] that has 2 pairs.
- [4,3,2,2,4] that has 2 pairs.
 

Constraints:

1 <= nums.length <= 105
1 <= nums[i], k <= 109



*/

use std::collections::HashMap;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        /*
        3 
        0 3 4 
        0 3 
        0 4 
        3 4
        */
        let mut left = 0;
        let mut count: i64 = 0;
        let mut freq = HashMap::new();
        let mut pairs: usize = 0;

        for right in 0..nums.len() {
            let entry = freq.entry(nums[right]).or_insert(0);
            pairs += *entry;
            *entry += 1;

            while pairs >= k as usize {
                count += (nums.len() - right) as i64;

                let left_val = nums[left];
                let entry = freq.get_mut(&left_val).unwrap();
                *entry -= 1;
                pairs -= *entry;

                left += 1;
            }
        }

        count
    }
}
