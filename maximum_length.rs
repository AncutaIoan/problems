/*
3201. Find the Maximum Length of Valid Subsequence I

You are given an integer array nums.
A subsequence sub of nums with length x is called valid if it satisfies:

(sub[0] + sub[1]) % 2 == (sub[1] + sub[2]) % 2 == ... == (sub[x - 2] + sub[x - 1]) % 2.
Return the length of the longest valid subsequence of nums.

A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
*/
impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let k: i32 = 2; // explicitly set k as i32
        let mut frequency = vec![vec![0; k as usize]; k as usize];
        let mut max_length = 0;

        for &num in &nums {
            let num_mod = num % k;
            for j in 0..k {
                let y = (j - num_mod + k) % k;
                frequency[num_mod as usize][y as usize] = frequency[y as usize][num_mod as usize] + 1;
                max_length = max_length.max(frequency[num_mod as usize][y as usize]);
            }
        }

        max_length
    }
}
