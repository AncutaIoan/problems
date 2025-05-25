/*
2131. Longest Palindrome by Concatenating Two Letter Words
You are given an array of strings words. Each element of words consists of two lowercase English letters.

Create the longest possible palindrome by selecting some elements from words and concatenating them in any order. Each element can be selected at most once.

Return the length of the longest palindrome that you can create. If it is impossible to create any palindrome, return 0.

A palindrome is a string that reads the same forward and backward.

 


*/
use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut counter = HashMap::new();

        for word in &words {
            *counter.entry(word.clone()).or_insert(0) += 1;
        }

        let mut length = 0;
        let mut center_used = false;

        for (word, &count) in counter.clone().iter() {
            let rev: String = word.chars().rev().collect();

            if word == &rev {
                // Palindromic word like "cc", "ll"
                let pairs = count / 2;
                length += pairs * 4;
                if count % 2 == 1 && !center_used {
                    length += 2;
                    center_used = true;
                }
            } else if word < &rev && counter.contains_key(&rev) {
                let rev_count = *counter.get(&rev).unwrap();
                let pairs = count.min(rev_count);
                length += pairs * 4;
            }
        }

        length
    }
}
