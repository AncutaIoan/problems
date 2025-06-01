/*
2929. Distribute Candies Among Children II

You are given two positive integers n and limit.

Return the total number of ways to distribute n candies among 3 children such that no child gets more than limit candies.

 

Example 1:

Input: n = 5, limit = 2
Output: 3
Explanation: There are 3 ways to distribute 5 candies such that no child gets more than 2 candies: (1, 2, 2), (2, 1, 2) and (2, 2, 1).
Example 2:

Input: n = 3, limit = 3
Output: 10
Explanation: There are 10 ways to distribute 3 candies such that no child gets more than 3 candies: (0, 0, 3), (0, 1, 2), (0, 2, 1), (0, 3, 0), (1, 0, 2), (1, 1, 1), (1, 2, 0), (2, 0, 1), (2, 1, 0) and (3, 0, 0).
 

Constraints:

1 <= n <= 106
1 <= limit <= 106

*/

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        fn comb2(x: i32) -> i64 {
            if x < 0 { 0 } else { (x as i64 + 1) * (x as i64 + 2) / 2 }
        }

        let total = comb2(n);

        let over_x = comb2(n - (limit + 1));
        let over_y = over_x;
        let over_z = over_x;

        let over_xy = comb2(n - 2 * (limit + 1));
        let over_xz = over_xy;
        let over_yz = over_xy;

        let over_xyz = comb2(n - 3 * (limit + 1));

        total - (over_x + over_y + over_z) + (over_xy + over_xz + over_yz) - over_xyz
    }
}
