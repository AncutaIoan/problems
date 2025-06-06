/*
768. Max Chunks To Make Sorted II

You are given an integer array arr.

We split arr into some number of chunks (i.e., partitions), and individually sort each chunk. After concatenating them, the result should equal the sorted array.

Return the largest number of chunks we can make to sort the array.

 

Example 1:

Input: arr = [5,4,3,2,1]
Output: 1
Explanation:
Splitting into two or more chunks will not return the required result.
For example, splitting into [5, 4], [3, 2, 1] will result in [4, 5, 1, 2, 3], which isn't sorted.
Example 2:

Input: arr = [2,1,3,4,4]
Output: 4
Explanation:
We can split into two chunks, such as [2, 1], [3, 4, 4].
However, splitting into [2, 1], [3], [4], [4] is the highest number of chunks possible.
 

Constraints:

1 <= arr.length <= 2000
0 <= arr[i] <= 108

*/
impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut left_max = vec![0; n];
        let mut right_min = vec![0; n + 1];

        left_max[0] = arr[0];
        for i in 1..n {
            left_max[i] = left_max[i - 1].max(arr[i]);
        }

        right_min[n] = i32::MAX;
        for i in (0..n).rev() {
            right_min[i] = right_min[i + 1].min(arr[i]);
        }

        let mut chunks = 0;
        for i in 0..n {
            if left_max[i] <= right_min[i + 1] {
                chunks += 1;
            }
        }

        chunks
    }
}
