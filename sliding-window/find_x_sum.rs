/*
3318. Find X-Sum of All K-Long Subarrays I

You are given an array nums of n integers and two integers k and x.

The x-sum of an array is calculated by the following procedure:

Count the occurrences of all elements in the array.
Keep only the occurrences of the top x most frequent elements. If two elements have the same number of occurrences, the element with the bigger value is considered more frequent.
Calculate the sum of the resulting array.
Note that if an array has less than x distinct elements, its x-sum is the sum of the array.

Return an integer array answer of length n - k + 1 where answer[i] is the x-sum of the subarray nums[i..i + k - 1].

 

Example 1:

Input: nums = [1,1,2,2,3,4,2,3], k = 6, x = 2

Output: [6,10,12]

Explanation:

For subarray [1, 1, 2, 2, 3, 4], only elements 1 and 2 will be kept in the resulting array. Hence, answer[0] = 1 + 1 + 2 + 2.
For subarray [1, 2, 2, 3, 4, 2], only elements 2 and 4 will be kept in the resulting array. Hence, answer[1] = 2 + 2 + 2 + 4. Note that 4 is kept in the array since it is bigger than 3 and 1 which occur the same number of times.
For subarray [2, 2, 3, 4, 2, 3], only elements 2 and 3 are kept in the resulting array. Hence, answer[2] = 2 + 2 + 2 + 3 + 3.
Example 2:

Input: nums = [3,8,7,8,7,5], k = 2, x = 2

Output: [11,15,15,15,12]

Explanation:

Since k == x, answer[i] is equal to the sum of the subarray nums[i..i + k - 1].

 

Constraints:

1 <= n == nums.length <= 50
1 <= nums[i] <= 50
1 <= x <= k <= nums.length
*/
use std::collections::HashMap;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let x = x as usize;
        let n = nums.len();
        let mut res = Vec::new();
        let mut freq = HashMap::new();

        for i in 0..n {
            // Add incoming element
            *freq.entry(nums[i]).or_insert(0) += 1;

            // Remove outgoing element if window too large
            if i >= k {
                let out = nums[i - k];
                if let Some(v) = freq.get_mut(&out) {
                    *v -= 1;
                    if *v == 0 {
                        freq.remove(&out);
                    }
                }
            }

            // Compute x-sum once window has size k
            if i + 1 >= k {
                let mut vec: Vec<(i32, i32)> = freq.iter().map(|(&num, &count)| (num, count)).collect();
                vec.sort_by(|a, b| {
                    b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0)) // sort by freq desc, then num desc
                });
                let mut sum = 0;
                for (num, count) in vec.into_iter().take(x) {
                    sum += num * count;
                }
                res.push(sum);
            }
        }

        res
    }
}
