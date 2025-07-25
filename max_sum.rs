/*
3487. Maximum Unique Subarray Sum After Deletion

You are given an integer array nums.

You are allowed to delete any number of elements from nums without making it empty. After performing the deletions, select a subarray of nums such that:

All elements in the subarray are unique.
The sum of the elements in the subarray is maximized.
Return the maximum sum of such a subarray.
*/
impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        // Find the maximum value in the array
        let max = *nums.iter().max().unwrap();

        // If the maximum value is less than or equal to 0, return it
        if max <= 0 {
            return max;
        }

        // Boolean array to keep track of seen positive numbers
        let mut seen = vec![false; 201]; // Range 0 to 200
        let mut result = 0;

        for &num in &nums {
            if num < 0 {
                continue;
            }

            if num <= 200 && !seen[num as usize] {
                result += num;
                seen[num as usize] = true;
            }
        }

        result
    }
}
