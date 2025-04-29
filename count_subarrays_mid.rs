/*
2962. Count Subarrays Where Max Element Appears at Least K Times
You are given an integer array nums and a positive integer k.
Return the number of subarrays where the maximum element of nums appears at least k times in that subarray.
A subarray is a contiguous sequence of elements within an array.

Example 1:

Input: nums = [1,3,2,3,3], k = 2
Output: 6
Explanation: The subarrays that contain the element 3 at least 2 times are: [1,3,2,3], [1,3,2,3,3], [3,2,3], [3,2,3,3], [2,3,3] and [3,3].
Example 2:

Input: nums = [1,4,2,1], k = 3
Output: 0
Explanation: No subarray contains the element 4 at least 3 times.
 

Constraints:

1 <= nums.length <= 105
1 <= nums[i] <= 106
1 <= k <= 105
*/

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max_val = *nums.iter().max().unwrap();
        let mut count = 0;
        let mut left = 0;
        let mut max_count = 0;
        let mut result = 0i64;

        for right in 0..nums.len() {
            if nums[right] == max_val {
                max_count += 1;
            }

            while max_count >= k {
                result += (nums.len() - right) as i64;
                if nums[left] == max_val {
                    max_count -= 1;
                }
                left += 1;
            }
        }

        result
    }
}
