/*
2561. Rearranging Fruits

You have two fruit baskets containing n fruits each. You are given two 0-indexed integer arrays basket1 and basket2 representing the cost of fruit in each basket. You want to make both baskets equal. To do so, you can use the following operation as many times as you want:

Chose two indices i and j, and swap the ith fruit of basket1 with the jth fruit of basket2.
The cost of the swap is min(basket1[i],basket2[j]).
Two baskets are considered equal if sorting them according to the fruit cost makes them exactly the same baskets.

Return the minimum cost to make both the baskets equal or -1 if impossible.

 

Example 1:

Input: basket1 = [4,2,2,2], basket2 = [1,4,1,2]
Output: 1
Explanation: Swap index 1 of basket1 with index 0 of basket2, which has cost 1. Now basket1 = [4,1,2,2] and basket2 = [2,4,1,2]. Rearranging both the arrays makes them equal.
Example 2:

Input: basket1 = [2,3,4,1], basket2 = [3,2,5,1]
Output: -1
Explanation: It can be shown that it is impossible to make both the baskets equal.
 

Constraints:

basket1.length == basket2.length
1 <= basket1.length <= 105
1 <= basket1[i],basket2[i] <= 109
*/
use std::collections::HashMap;

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let mut count = HashMap::new();
        let mut freq1 = HashMap::new();
        let mut freq2 = HashMap::new();

        for &x in &basket1 {
            *count.entry(x).or_insert(0) += 1;
            *freq1.entry(x).or_insert(0) += 1;
        }
        for &x in &basket2 {
            *count.entry(x).or_insert(0) += 1;
            *freq2.entry(x).or_insert(0) += 1;
        }

        // Check if it's possible to make them equal
        for &v in count.values() {
            if v % 2 != 0 {
                return -1;
            }
        }

        // Find the global minimum fruit cost
        let min_val = *count.keys().min().unwrap();

        // Build the list of mismatches
        let mut to_swap = Vec::new();
        for (&fruit, &total) in count.iter() {
            let f1 = *freq1.get(&fruit).unwrap_or(&0);
            let f2 = *freq2.get(&fruit).unwrap_or(&0);
            let diff: i32 = f1 - f2;
            if diff != 0 {
                for _ in 0..(diff.abs() / 2) {
                    to_swap.push(fruit);
                }
            }
        }

        // Only need to sort half and use min swap cost
        to_swap.sort();
        let swaps = to_swap.len() / 2;
        let mut result: i64 = 0;
        for i in 0..swaps {
            result += i64::from(to_swap[i].min(2 * min_val));
        }

        result
    }
}
