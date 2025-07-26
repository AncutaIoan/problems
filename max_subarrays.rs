/*
3480. Maximize Subarrays After Removing One Conflicting Pair

You are given an integer n which represents an array nums containing the numbers from 1 to n in order. Additionally, you are given a 2D array conflictingPairs, where conflictingPairs[i] = [a, b] indicates that a and b form a conflicting pair.

Remove exactly one element from conflictingPairs. Afterward, count the number of non-empty subarrays of nums which do not contain both a and b for any remaining conflicting pair [a, b].

Return the maximum number of subarrays possible after removing exactly one conflicting pair.
*/
impl Solution {
    pub fn max_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut valid_subarrays: i64 = 0;
        let mut max_left: usize = 0;
        let mut second_max_left: usize = 0;
        let mut gains = vec![0i64; n + 1];
        let mut conflicts = vec![Vec::new(); n + 1];

        for pair in conflicting_pairs {
            let a = pair[0] as usize;
            let b = pair[1] as usize;
            let right = a.max(b);
            let left = a.min(b);
            conflicts[right].push(left);
        }

        for right in 1..=n {
            for &left in &conflicts[right] {
                if left > max_left {
                    second_max_left = max_left;
                    max_left = left;
                } else if left > second_max_left {
                    second_max_left = left;
                }
            }

            valid_subarrays += (right - max_left) as i64;
            gains[max_left] += (max_left - second_max_left) as i64;
        }

        valid_subarrays + *gains.iter().max().unwrap()
    }
}
