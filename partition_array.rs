/*
2294. Partition Array Such That Maximum Difference Is K

You are given an integer array nums and an integer k. You may partition nums into one or more subsequences such that each element in nums appears in exactly one of the subsequences.

Return the minimum number of subsequences needed such that the difference between the maximum and minimum values in each subsequence is at most k.

A subsequence is a sequence that can be derived from another sequence by deleting some or no elements without changing the order of the remaining elements.

*/
impl Solution {
    pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut count = 0;
        let mut i = 0;

        while i < nums.len() {
            count += 1;
            let start = nums[i];
            // Keep extending the current subsequence as long as condition holds
            while i < nums.len() && nums[i] - start <= k {
                i += 1;
            }
        }

        count
    }
}
