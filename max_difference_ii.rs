/*
3445. Maximum Difference Between Even and Odd Frequency II
 
You are given a string s and an integer k. Your task is to find the maximum difference between the frequency of two characters, freq[a] - freq[b], in a substring subs of s, such that:

subs has a size of at least k.
Character a has an odd frequency in subs.
Character b has an even frequency in subs.
Return the maximum difference.

Note that subs can contain more than 2 distinct characters.

*/

use std::cmp::min;

impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
        let chars: Vec<u8> = s.bytes().collect();
        let n = chars.len();
        let digits = b"01234";
        let mut ans = i32::MIN;

        for &a in digits {
            for &b in digits {
                if a == b {
                    continue;
                }

                let mut prefix_a = vec![0];
                let mut prefix_b = vec![0];

                for &ch in &chars {
                    prefix_a.push(prefix_a.last().unwrap() + if ch == a { 1 } else { 0 });
                    prefix_b.push(prefix_b.last().unwrap() + if ch == b { 1 } else { 0 });
                }

                // min_diff[parity_a][parity_b] = minimum a_count - b_count seen with that parity
                let mut min_diff = [[i32::MAX / 2; 2]; 2];
                let mut l = 0;

                for r in 0..n {
                    while r - l + 1 >= k as usize
                        && prefix_a[l] < prefix_a[r + 1]
                        && prefix_b[l] < prefix_b[r + 1]
                    {
                        let pa = prefix_a[l] % 2;
                        let pb = prefix_b[l] % 2;
                        let diff = prefix_a[l] as i32 - prefix_b[l] as i32;
                        min_diff[pa][pb] = min(min_diff[pa][pb], diff);
                        l += 1;
                    }

                    let cur_a = prefix_a[r + 1] as i32;
                    let cur_b = prefix_b[r + 1] as i32;
                    let pa = (cur_a % 2) as usize;
                    let pb = (cur_b % 2) as usize;
                    let opposite_pa = 1 - pa;
                    let min_val = min_diff[opposite_pa][pb];

                    if min_val < i32::MAX / 2 {
                        ans = ans.max(cur_a - cur_b - min_val);
                    }
                }
            }
        }

        ans
    }
}
