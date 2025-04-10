/*
2999. Count the Number of Powerful Integers
You are given three integers start, finish, and limit. You are also given a 0-indexed string s representing a positive integer.

A positive integer x is called powerful if it ends with s (in other words, s is a suffix of x) and each digit in x is at most limit.

Return the total number of powerful integers in the range [start..finish].

A string x is a suffix of a string y if and only if x is a substring of y that starts from some index (including 0) in y and extends to the index y.length - 1. For example, 25 is a suffix of 5125 whereas 512 is not.

 


*/



use std::collections::HashMap;

impl Solution {
    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        fn count_up_to(bound: i64, limit: i32, s: &str) -> i64 {
            let t: Vec<u8> = bound.to_string().bytes().map(|b| b - b'0').collect();
            let n = s.len();
            let s_bytes: Vec<u8> = s.bytes().collect();
            let mut memo = HashMap::new();

            fn dfs(
                pos: usize,
                lim: bool,
                t: &Vec<u8>,
                limit: u8,
                s_bytes: &Vec<u8>,
                n: usize,
                memo: &mut HashMap<(usize, bool), i64>,
            ) -> i64 {
                if t.len() < n {
                    return 0;
                }

                if t.len() - pos == n {
                    if !lim {
                        return 1;
                    } else {
                        // Compare s to the suffix starting from t[pos..]
                        for i in 0..n {
                            let tc = t[pos + i];
                            let sc = s_bytes[i] - b'0';
                            if tc < sc {
                                return 0;
                            } else if tc > sc {
                                return 1;
                            }
                        }
                        return 1;
                    }
                }

                if let Some(&cached) = memo.get(&(pos, lim)) {
                    return cached;
                }

                let up = if lim { t[pos].min(limit) } else { limit };
                let mut ans = 0;

                for i in 0..=up {
                    let next_lim = lim && i == t[pos];
                    ans += dfs(pos + 1, next_lim, t, limit, s_bytes, n, memo);
                }

                memo.insert((pos, lim), ans);
                ans
            }

            dfs(0, true, &t, limit as u8, &s_bytes, n, &mut memo)
        }

        count_up_to(finish, limit, &s) - count_up_to(start - 1, limit, &s)
    }
}
