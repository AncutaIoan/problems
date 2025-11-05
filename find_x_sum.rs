/*
3321. Find X-Sum of All K-Long Subarrays II

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

nums.length == n
1 <= n <= 105
1 <= nums[i] <= 109
1 <= x <= k <= nums.length
*/
use std::collections::{BTreeSet, HashMap};

#[derive(Eq, PartialEq, Clone, Copy)]
struct Elem {
    freq: i32,
    val: i32,
}

// Custom ordering: higher freq first, if tie → higher val first
impl Ord for Elem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.freq.cmp(&self.freq).then_with(|| other.val.cmp(&self.val))
    }
}
impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        let (k, x) = (k as usize, x as usize);
        let mut res = Vec::new();
        let mut freq = HashMap::new();

        let mut top = BTreeSet::new();    // top x elements
        let mut rest = BTreeSet::new();   // remaining elements
        let mut sum_top: i64 = 0;

        // helper: insert element and maintain balance
        let mut insert = |freq: &mut HashMap<i32, i32>,
                          top: &mut BTreeSet<Elem>,
                          rest: &mut BTreeSet<Elem>,
                          sum_top: &mut i64,
                          val: i32, delta: i32, x: usize| {
            // remove old entry if exists
            let old_freq = *freq.get(&val).unwrap_or(&0);
            if old_freq > 0 {
                let e = Elem { freq: old_freq, val };
                if top.remove(&e) {
                    *sum_top -= (e.freq as i64) * (e.val as i64);
                } else {
                    rest.remove(&e);
                }
            }

            // update frequency
            let new_freq = old_freq + delta;
            if new_freq == 0 {
                freq.remove(&val);
            } else {
                freq.insert(val, new_freq);
                let e = Elem { freq: new_freq, val };
                rest.insert(e);
            }

            // rebalance: move best elements to top until top has x
            while top.len() < x && !rest.is_empty() {
                let best = *rest.iter().next().unwrap();
                rest.remove(&best);
                *sum_top += (best.freq as i64) * (best.val as i64);
                top.insert(best);
            }

            // if rest’s best beats top’s worst, swap
            if !rest.is_empty() && !top.is_empty() {
                let best_rest = *rest.iter().next().unwrap();
                let worst_top = *top.iter().rev().next().unwrap();
                if best_rest < worst_top {
                    // swap
                    rest.remove(&best_rest);
                    top.remove(&worst_top);
                    *sum_top += (best_rest.freq as i64) * (best_rest.val as i64);
                    *sum_top -= (worst_top.freq as i64) * (worst_top.val as i64);
                    rest.insert(worst_top);
                    top.insert(best_rest);
                }
            }
        };

        for i in 0..nums.len() {
            insert(&mut freq, &mut top, &mut rest, &mut sum_top, nums[i], 1, x);
            if i >= k {
                insert(&mut freq, &mut top, &mut rest, &mut sum_top, nums[i - k], -1, x);
            }
            if i + 1 >= k {
                res.push(sum_top);
            }
        }

        res
    }
}
