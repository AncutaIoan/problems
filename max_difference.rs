/*
3442. Maximum Difference Between Even and Odd Frequency I
You are given a string s consisting of lowercase English letters.

Your task is to find the maximum difference diff = freq(a1) - freq(a2) between the frequency of characters a1 and a2 in the string such that:

a1 has an odd frequency in the string.
a2 has an even frequency in the string.
Return this maximum difference.

*/

use std::collections::HashMap;

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut freq_map = HashMap::new();

        // Count the frequency of each character
        for ch in s.chars() {
            *freq_map.entry(ch).or_insert(0) += 1;
        }

        let mut odd_freqs = Vec::new();
        let mut even_freqs = Vec::new();

        // Separate odd and even frequencies
        for &freq in freq_map.values() {
            if freq % 2 == 1 {
                odd_freqs.push(freq);
            } else {
                even_freqs.push(freq);
            }
        }

        // If we don't have both types, return 0
        if odd_freqs.is_empty() || even_freqs.is_empty() {
            return 0;
        }

        // Compute maximum difference
        let mut max_diff = i32::MIN;
        for &odd in &odd_freqs {
            for &even in &even_freqs {
                let diff = odd - even;
                if diff > max_diff {
                    max_diff = diff;
                }
            }
        }

        max_diff
    }
}
