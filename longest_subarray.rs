/*
2419. Longest Subarray With Maximum Bitwise AND

You are given an integer array nums of size n.

Consider a non-empty subarray from nums that has the maximum possible bitwise AND.

In other words, let k be the maximum value of the bitwise AND of any subarray of nums. Then, only subarrays with a bitwise AND equal to k should be considered.
Return the length of the longest such subarray.

The bitwise AND of an array is the bitwise AND of all the numbers in it.

A subarray is a contiguous sequence of elements within an array.


*/
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let max_val = *nums.iter().max().unwrap();
        let mut max_len = 0;
        let mut current_len = 0;

        for &num in nums.iter() {
            if num == max_val {
                current_len += 1;
                max_len = max_len.max(current_len);
            } else {
                current_len = 0;
            }
        }

        max_len
    }
}
