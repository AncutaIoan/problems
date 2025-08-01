/*
81. Search in Rotated Sorted Array II

There is an integer array nums sorted in non-decreasing order (not necessarily with distinct values).

Before being passed to your function, nums is rotated at an unknown pivot index k (0 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,4,4,5,6,6,7] might be rotated at pivot index 5 and become [4,5,6,6,7,0,1,2,4,4].

Given the array nums after the rotation and an integer target, return true if target is in nums, or false if it is not in nums.

You must decrease the overall operation steps as much as possible.

 

Example 1:

Input: nums = [2,5,6,0,0,1,2], target = 0
Output: true
Example 2:

Input: nums = [2,5,6,0,0,1,2], target = 3
Output: false
 

*/

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let (mut left, mut right) = (0, nums.len().saturating_sub(1));

        while left <= right {
            let mid = left + (right - left) / 2;

            if nums[mid] == target {
                return true;
            }

            // Handle duplicates — we can’t determine sorted half
            if nums[left] == nums[mid] && nums[right] == nums[mid] {
                left += 1;
                right = right.saturating_sub(1);
            } 
            // Left half is sorted
            else if nums[left] <= nums[mid] {
                if nums[left] <= target && target < nums[mid] {
                    right = mid.saturating_sub(1);
                } else {
                    left = mid + 1;
                }
            } 
            // Right half is sorted
            else {
                if nums[mid] < target && target <= nums[right] {
                    left = mid + 1;
                } else {
                    right = mid.saturating_sub(1);
                }
            }
        }

        false
    }
}
