/*
368. Largest Divisible Subset

Given a set of distinct positive integers nums, return the largest subset answer such that every pair (answer[i], answer[j]) of elements in this subset satisfies:

answer[i] % answer[j] == 0, or
answer[j] % answer[i] == 0
If there are multiple solutions, return any of them.

 

Example 1:

Input: nums = [1,2,3]
Output: [1,2]
Explanation: [1,3] is also accepted.
Example 2:

Input: nums = [1,2,4,8]
Output: [1,2,4,8]


*/


impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }

        nums.sort();

        let n = nums.len();
        let mut dp = vec![1; n]; // Length of the largest subset ending at i
        let mut prev = vec![-1; n]; // Index of previous element in the subset
        let mut max_index = 0;

        for i in 1..n {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && dp[j] + 1 > dp[i] {
                    dp[i] = dp[j] + 1;
                    prev[i] = j as i32;
                }
            }
            if dp[i] > dp[max_index] {
                max_index = i;
            }
        }

        let mut result = Vec::new();
        let mut k = max_index as i32;
        while k >= 0 {
            result.push(nums[k as usize]);
            k = prev[k as usize];
        }

        result.reverse();
        result
    }
}
