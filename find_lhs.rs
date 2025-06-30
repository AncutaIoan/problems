/*
594. Longest Harmonious Subsequence

We define a harmonious array as an array where the difference between its maximum value and its minimum value is exactly 1.

Given an integer array nums, return the length of its longest harmonious subsequence among all its possible subsequences.
*/
impl Solution {
    pub fn find_lhs(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut i = 0;
        let mut max_len = 0;

        for j in 0..nums.len() {
            while nums[j] - nums[i] > 1 {
                i += 1;
            }
            if nums[j] - nums[i] == 1 {
                max_len = max_len.max(j - i + 1);
            }
        }

        max_len as i32
    }
}
