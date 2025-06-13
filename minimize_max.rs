/*
2616. Minimize the Maximum Difference of Pairs

You are given a 0-indexed integer array nums and an integer p. Find p pairs of indices of nums such that the maximum difference amongst all the pairs is minimized. Also, ensure no index appears more than once amongst the p pairs.

Note that for a pair of elements at the index i and j, the difference of this pair is |nums[i] - nums[j]|, where |x| represents the absolute value of x.

Return the minimum maximum difference among all p pairs. We define the maximum of an empty set to be zero.
*/

impl Solution {
    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        nums.sort_unstable();
        
        let mut low = 0;
        let mut high = nums[nums.len() - 1] - nums[0];
        
        while low < high {
            let mid = low + (high - low) / 2;
            
            if Self::can_form_pairs(&nums, p, mid) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        
        low
    }
    
    fn can_form_pairs(nums: &Vec<i32>, p: i32, max_diff: i32) -> bool {
        let mut count = 0;
        let mut i = 0;
        
        while i < nums.len() - 1 {
            if nums[i + 1] - nums[i] <= max_diff {
                count += 1;
                i += 2; // skip both elements in the pair
            } else {
                i += 1;
            }
            
            if count >= p {
                return true;
            }
        }
        
        false
    }
}
