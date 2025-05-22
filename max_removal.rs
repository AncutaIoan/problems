/*

3362. Zero Array Transformation III

You are given an integer array nums of length n and a 2D array queries where queries[i] = [li, ri].

Each queries[i] represents the following action on nums:

Decrement the value at each index in the range [li, ri] in nums by at most 1.
The amount by which the value is decremented can be chosen independently for each index.
A Zero Array is an array with all its elements equal to 0.

Return the maximum number of elements that can be removed from queries, such that nums can still be converted to a zero array using the remaining queries. If it is not possible to convert nums to a zero array, return -1.

 

Example 1:

Input: nums = [2,0,2], queries = [[0,2],[0,2],[1,1]]

Output: 1

Explanation:

After removing queries[2], nums can still be converted to a zero array.

Using queries[0], decrement nums[0] and nums[2] by 1 and nums[1] by 0.
Using queries[1], decrement nums[0] and nums[2] by 1 and nums[1] by 0.
Example 2:

Input: nums = [1,1,1,1], queries = [[1,3],[0,2],[1,3],[1,2]]

Output: 2

Explanation:

We can remove queries[2] and queries[3].

Example 3:

Input: nums = [1,2,3,4], queries = [[0,3]]

Output: -1

Explanation:

nums cannot be converted to a zero array even after using all the queries.
*/

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
        let mut result = queries.len() as i32;

        queries.sort_unstable();

        let mut queries_pq = BinaryHeap::new();
        let mut query_i = 0;

        let mut increment_pq = BinaryHeap::new();

        let mut decrement = 0;

        for (num_i, mut num) in nums.into_iter().enumerate() {
            while let Some(&Reverse(i)) = increment_pq.peek() {
                if i > num_i {
                    break;
                }

                decrement -= 1;
                increment_pq.pop();
            }

            num -= decrement;

            while let Some(query) = queries.get(query_i) {
                if query[0] as usize > num_i {
                    break;
                }

                queries_pq.push(query[1] as usize + 1);
                query_i += 1;
            }

            while num > 0 {
                increment_pq.push(Reverse(match queries_pq.pop() {
                    Some(x) if x > num_i => x,
                    _ => return -1,
                }));

                num -= 1;
                decrement += 1;
                result -= 1;
            }
        }

        result
    }
}
