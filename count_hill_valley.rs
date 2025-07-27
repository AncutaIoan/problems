/*
2210. Count Hills and Valleys in an Array
You are given a 0-indexed integer array nums. An index i is part of a hill in nums if the closest non-equal neighbors of i are smaller than nums[i]. Similarly, an index i is part of a valley in nums if the closest non-equal neighbors of i are larger than nums[i]. Adjacent indices i and j are part of the same hill or valley if nums[i] == nums[j].

Note that for an index to be part of a hill or valley, it must have a non-equal neighbor on both the left and right of the index.

Return the number of hills and valleys in nums.
*/
impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut i = 1;

        while i < nums.len() - 1 {
            let mut left = i - 1;
            // Skip equal values to the left
            while left > 0 && nums[left] == nums[i] {
                left -= 1;
            }

            let mut right = i + 1;
            // Skip equal values to the right
            while right < nums.len() && nums[right] == nums[i] {
                right += 1;
            }

            if nums[left] < nums[i] && right < nums.len() && nums[right] < nums[i] {
                count += 1; // Hill
            } else if nums[left] > nums[i] && right < nums.len() && nums[right] > nums[i] {
                count += 1; // Valley
            }

            i = right; // Skip over the plateau
        }

        count
    }
}
