/*
2014. Longest Subsequence Repeated k Times

You are given a string s of length n, and an integer k. You are tasked to find the longest subsequence repeated k times in string s.

A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.

A subsequence seq is repeated k times in the string s if seq * k is a subsequence of s, where seq * k represents a string constructed by concatenating seq k times.

For example, "bba" is repeated 2 times in the string "bababcba", because the string "bbabba", constructed by concatenating "bba" 2 times, is a subsequence of the string "bababcba".
Return the longest subsequence repeated k times in string s. If multiple such subsequences are found, return the lexicographically largest one. If there is no such subsequence, return an empty string.

*/

use std::collections::{VecDeque, HashMap};

impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        fn is_k_repeated_subseq(s: &Vec<char>, target: &Vec<char>, k: i32) -> bool {
            let mut ti = 0;
            let mut repeat = 0;

            for &c in s.iter() {
                if c == target[ti] {
                    ti += 1;
                    if ti == target.len() {
                        repeat += 1;
                        ti = 0;
                        if repeat == k {
                            return true;
                        }
                    }
                }
            }
            false
        }

        let s_chars: Vec<char> = s.chars().collect();
        let mut freq = HashMap::new();

        for &c in &s_chars {
            *freq.entry(c).or_insert(0) += 1;
        }

        // Characters that appear at least k times are usable
        let mut valid_chars: Vec<char> = freq.iter()
            .filter(|(_, &v)| v >= k)
            .map(|(&c, _)| c)
            .collect();

        valid_chars.sort_by(|a, b| b.cmp(a)); // lexicographically larger first

        let mut queue = VecDeque::new();
        queue.push_back(Vec::new());
        let mut result = Vec::new();

        while let Some(cur) = queue.pop_front() {
            for &c in &valid_chars {
                let mut next = cur.clone();
                next.push(c);
                if is_k_repeated_subseq(&s_chars, &next, k) {
                    if next.len() > result.len() || (next.len() == result.len() && next > result) {
                        result = next.clone();
                    }
                    queue.push_back(next);
                }
            }
        }

        result.iter().collect()
    }
}
