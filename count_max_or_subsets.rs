/*
2044. Count Number of Maximum Bitwise-OR Subsets
Solved
Medium

Topics
premium lock icon
Companies

Hint
Given an integer array nums, find the maximum possible bitwise OR of a subset of nums and return the number of different non-empty subsets with the maximum bitwise OR.

An array a is a subset of an array b if a can be obtained from b by deleting some (possibly zero) elements of b. Two subsets are considered different if the indices of the elements chosen are different.

The bitwise OR of an array a is equal to a[0] OR a[1] OR ... OR a[a.length - 1] (0-indexed).
*/
impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        fn dfs(nums: &Vec<i32>, index: usize, current_or: i32, max_or: i32, count: &mut i32, max_val: &mut i32) {
            if index == nums.len() {
                if current_or == *max_val {
                    *count += 1;
                } else if current_or > *max_val {
                    *max_val = current_or;
                    *count = 1;
                }
                return;
            }
            
            // Include nums[index]
            dfs(nums, index + 1, current_or | nums[index], max_or, count, max_val);
            
            // Exclude nums[index]
            dfs(nums, index + 1, current_or, max_or, count, max_val);
        }

        let mut count = 0;
        let mut max_or = 0;
        dfs(&nums, 0, 0, 0, &mut count, &mut max_or);
        count
    }
}
