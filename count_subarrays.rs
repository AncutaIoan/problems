/*
2444. Count Subarrays With Fixed Bounds

You are given an integer array nums and two integers minK and maxK.

A fixed-bound subarray of nums is a subarray that satisfies the following conditions:

The minimum value in the subarray is equal to minK.
The maximum value in the subarray is equal to maxK.
Return the number of fixed-bound subarrays.

A subarray is a contiguous part of an array.

 

Example 1:

Input: nums = [1,3,5,2,7,5], minK = 1, maxK = 5
Output: 2
Explanation: The fixed-bound subarrays are [1,3,5] and [1,3,5,2].
Example 2:

Input: nums = [1,1,1,1], minK = 1, maxK = 1
Output: 10
Explanation: Every subarray of nums is a fixed-bound subarray. There are 10 possible subarrays.
 

Constraints:

2 <= nums.length <= 105
1 <= nums[i], minK, maxK <= 106
*/
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut last_min = -1;
        let mut last_max = -1;
        let mut last_invalid = -1;
        let mut result = 0i64;

        for (i, &num) in nums.iter().enumerate() {
            if num < min_k || num > max_k {
                last_invalid = i as i32;
            }
            if num == min_k {
                last_min = i as i32;
            }
            if num == max_k {
                last_max = i as i32;
            }
            let valid_start = last_min.min(last_max);
            if valid_start > last_invalid {
                result += (valid_start - last_invalid) as i64;
            }
        }

        result
    }
}
