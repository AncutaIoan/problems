/*
2099. Find Subsequence of Length K With the Largest Sum
You are given an integer array nums and an integer k. You want to find a subsequence of nums of length k that has the largest sum.

Return any such subsequence as an integer array of length k.

A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
*/
impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        
        // Collect nums with their indices
        let mut indexed: Vec<(usize, i32)> = nums.iter().enumerate().map(|(i, &num)| (i, num)).collect();
        
        // Sort by value descending to get the largest k elements
        indexed.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Take top k elements
        let mut top_k = indexed.into_iter().take(k).collect::<Vec<(usize, i32)>>();
        
        // Sort by index to maintain the subsequence order
        top_k.sort_by_key(|x| x.0);
        
        // Extract just the values
        top_k.into_iter().map(|(_, num)| num).collect()
    }
}
