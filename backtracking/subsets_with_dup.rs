/*
90. Subsets II

Given an integer array nums that may contain duplicates, return all possible subsets (the power set).

The solution set must not contain duplicate subsets. Return the solution in any order.
*/
impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort(); // Step 1: Sort to handle duplicates

        let mut result = Vec::new();
        let mut subset = Vec::new();

        fn backtrack(start: usize, nums: &Vec<i32>, subset: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            result.push(subset.clone());

            for i in start..nums.len() {
                // Step 3: Skip duplicates
                if i > start && nums[i] == nums[i - 1] {
                    continue;
                }

                subset.push(nums[i]);
                backtrack(i + 1, nums, subset, result);
                subset.pop();
            }
        }

        backtrack(0, &nums, &mut subset, &mut result);
        result
    }
}
