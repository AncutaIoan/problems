/*
2411. Smallest Subarrays With Maximum Bitwise OR

You are given a 0-indexed array nums of length n, consisting of non-negative integers. For each index i from 0 to n - 1, you must determine the size of the minimum sized non-empty subarray of nums starting at i (inclusive) that has the maximum possible bitwise OR.

In other words, let Bij be the bitwise OR of the subarray nums[i...j]. You need to find the smallest subarray starting at i, such that bitwise OR of this subarray is equal to max(Bik) where i <= k <= n - 1.
The bitwise OR of an array is the bitwise OR of all the numbers in it.

Return an integer array answer of size n where answer[i] is the length of the minimum sized subarray starting at i with maximum bitwise OR.

A subarray is a contiguous non-empty sequence of elements within an array.
*/
impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut answer = vec![0; n];
        let mut last_seen = vec![-1; 32]; // last seen positions for each bit
        
        for i in (0..n).rev() {
            // Update the last seen position for each bit in nums[i]
            for b in 0..32 {
                if (nums[i] >> b) & 1 == 1 {
                    last_seen[b] = i as i32;
                }
            }
            
            // Determine the furthest index we need to go to collect all bits
            let mut max_len = i as i32;
            for &pos in &last_seen {
                if pos != -1 {
                    max_len = max_len.max(pos);
                }
            }
            answer[i] = (max_len - i as i32 + 1) as i32;
        }
        
        answer
    }
}
