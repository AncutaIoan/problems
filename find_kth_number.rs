/*
440. K-th Smallest in Lexicographical Order

Given two integers n and k, return the kth lexicographically smallest integer in the range [1, n].

*/

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        fn calc_steps(n: i64, mut curr: i64, mut next: i64) -> i64 {
            let mut steps = 0;
            while curr <= n {
                steps += std::cmp::min(n + 1, next) - curr;
                curr *= 10;
                next *= 10;
            }
            steps
        }

        let mut k = k as i64 - 1; // Start from the first number (1)
        let mut curr = 1i64;

        while k > 0 {
            let steps = calc_steps(n as i64, curr, curr + 1);
            if steps <= k {
                curr += 1;
                k -= steps;
            } else {
                curr *= 10;
                k -= 1;
            }
        }

        curr as i32
    }
}
