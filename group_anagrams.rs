/*
49. Group Anagrams
Given an array of strings strs, group the anagrams together. You can return the answer in any order.
*/
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();
            let key = chars.into_iter().collect::<String>();
            map.entry(key).or_insert(Vec::new()).push(s);
        }

        map.into_values().collect()
    }
}
