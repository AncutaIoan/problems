/*
2163. Minimum Difference in Sums After Removal of Elements

You are given a 0-indexed integer array nums consisting of 3 * n elements.

You are allowed to remove any subsequence of elements of size exactly n from nums. The remaining 2 * n elements will be divided into two equal parts:

The first n elements belonging to the first part and their sum is sumfirst.
The next n elements belonging to the second part and their sum is sumsecond.
The difference in sums of the two parts is denoted as sumfirst - sumsecond.

For example, if sumfirst = 3 and sumsecond = 2, their difference is 1.
Similarly, if sumfirst = 2 and sumsecond = 3, their difference is -1.
Return the minimum difference possible between the sums of the two parts after the removal of n elements
*/
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let total_length = nums.len();
        let subset_size = total_length / 3;
        let mut sum: i64 = 0;

        // Prefix sums: for first 2/3 of the array
        let mut prefix_sums = vec![0i64; total_length + 1];
        let mut max_heap = BinaryHeap::new();

        for i in 1..=subset_size * 2 {
            let element = nums[i - 1];
            sum += element as i64;
            max_heap.push(element);

            if max_heap.len() > subset_size {
                if let Some(max_val) = max_heap.pop() {
                    sum -= max_val as i64;
                }
            }

            prefix_sums[i] = sum;
        }

        // Suffix sums: for last 2/3 of the array
        sum = 0;
        let mut suffix_sums = vec![0i64; total_length + 1];
        let mut min_heap = BinaryHeap::new();

        for i in (subset_size + 1..=total_length).rev() {
            let element = nums[i - 1];
            sum += element as i64;
            min_heap.push(Reverse(element));

            if min_heap.len() > subset_size {
                if let Some(Reverse(min_val)) = min_heap.pop() {
                    sum -= min_val as i64;
                }
            }

            suffix_sums[i] = sum;
        }

        // Calculate minimum difference
        let mut answer = i64::MAX;
        for i in subset_size..=subset_size * 2 {
            answer = answer.min(prefix_sums[i] - suffix_sums[i + 1]);
        }

        answer
    }
}
