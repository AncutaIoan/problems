/*
2221. Find Triangular Sum of an Array

You are given a 0-indexed integer array nums, where nums[i] is a digit between 0 and 9 (inclusive).

The triangular sum of nums is the value of the only element present in nums after the following process terminates:

Let nums comprise of n elements. If n == 1, end the process. Otherwise, create a new 0-indexed integer array newNums of length n - 1.
For each index i, where 0 <= i < n - 1, assign the value of newNums[i] as (nums[i] + nums[i+1]) % 10, where % denotes modulo operator.
Replace the array nums with newNums.
Repeat the entire process starting from step 1.
Return the triangular sum of nums.
*/
impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![0; n]; n];

        for i in 0..n {
            dp[0][i] = nums[i];
        }

        for i in 1..n {
            for j in 0..n-i {
                dp[i][j] = (dp[i-1][j] + dp[i-1][j + 1]) % 10;
            }
        }


        dp[n-1][0]
    }
}
