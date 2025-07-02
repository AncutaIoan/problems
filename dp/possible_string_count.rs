/*
3333. Find the Original Typed String II

Alice is attempting to type a specific string on her computer. However, she tends to be clumsy and may press a key for too long, resulting in a character being typed multiple times.

You are given a string word, which represents the final output displayed on Alice's screen. You are also given a positive integer k.

Return the total number of possible original strings that Alice might have intended to type, if she was trying to type a string of size at least k.

Since the answer may be very large, return it modulo 109 + 7.
*/

const M: i64 = 1_000_000_007;

impl Solution {
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        if k as usize > word.len() {
            return 0;
        }

        let chars: Vec<char> = word.chars().collect();
        let mut freq = Vec::new();
        let mut count = 1;

        for i in 1..chars.len() {
            if chars[i] == chars[i - 1] {
                count += 1;
            } else {
                freq.push(count);
                count = 1;
            }
        }
        freq.push(count);

        let mut p: i64 = 1;
        for &f in &freq {
            p = (p * f as i64) % M;
        }

        if freq.len() >= k as usize {
            return p as i32;
        }

        let n = freq.len();
        let mut t = vec![vec![0i64; k as usize + 1]; n + 1];

        for c in (0..k as usize).rev() {
            t[n][c] = 1;
        }

        for i in (0..n).rev() {
            let mut prefix = vec![0i64; k as usize + 2];
            for h in 1..=k as usize {
                prefix[h] = (prefix[h - 1] + t[i + 1][h - 1]) % M;
            }

            for c in (0..k as usize).rev() {
                let l = c + 1;
                let mut r = c + freq[i];
                if r + 1 > k as usize {
                    r = k as usize - 1;
                }

                if l <= r {
                    t[i][c] = (prefix[r + 1] - prefix[l] + M) % M;
                }
            }
        }

        let invalid_count = t[0][0];
        ((p - invalid_count + M) % M) as i32

    }
}
