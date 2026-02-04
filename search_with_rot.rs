/*
33. Search in Rotated Sorted Array

There is an integer array nums sorted in ascending order (with distinct values).

Prior to being passed to your function, nums is possibly left rotated at an unknown index k (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be left rotated by 3 indices and become [4,5,6,7,0,1,2].

Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.

You must write an algorithm with O(log n) runtime complexity.

 

Example 1:

Input: nums = [4,5,6,7,0,1,2], target = 0
Output: 4
Example 2:

Input: nums = [4,5,6,7,0,1,2], target = 3
Output: -1
Example 3:

Input: nums = [1], target = 0
Output: -1
 

Constraints:

1 <= nums.length <= 5000
-104 <= nums[i] <= 104
All values of nums are unique.
nums is an ascending array that is possibly rotated.
-104 <= target <= 104
*/
/*
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut rot_i: usize = 0;
        for i in 1..nums.len() {
            if nums[i] < nums[i-1] {
                rot_i = i;
                break;
            }
        }


        let n = nums.len();

        // decide which half to binary search
        if target >= nums[rot_i] && target <= nums[n - 1] {
            Solution::binary_search(&nums, rot_i, n - 1, target)
        } else {
            Solution::binary_search(&nums, 0, rot_i.saturating_sub(1), target)
        }



    }

    fn binary_search(nums: &Vec<i32>, left: usize, right: usize, target: i32) -> i32 {
        let mut l = left as i32;
        let mut r = right as i32;

        while l <= r {
            let mid = l + (r - l) / 2;
            let m = mid as usize;

            if nums[m] == target {
                return mid;
            } else if nums[m] < target {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        -1
    }

}
*/


impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_usize = mid as usize;

            if nums[mid_usize] == target {
                return mid;
            }

            // left half is sorted
            if nums[left as usize] <= nums[mid_usize] {
                if nums[left as usize] <= target && target < nums[mid_usize] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            // right half is sorted
            else {
                if nums[mid_usize] < target && target <= nums[right as usize] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }

        -1
    }
}
