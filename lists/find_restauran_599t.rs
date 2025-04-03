/*

599. Minimum Index Sum of Two Lists

Given two arrays of strings list1 and list2, find the common strings with the least index sum.

A common string is a string that appeared in both list1 and list2.

A common string with the least index sum is a common string such that if it appeared at list1[i] and list2[j] then i + j should be the minimum value among all the other common strings.

Return all the common strings with the least index sum. Return the answer in any order.




*/


use std::collections::HashMap;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut index_map = HashMap::new();
        
        // Store indices of list1 in hashmap
        for (i, word) in list1.iter().enumerate() {
            index_map.insert(word, i as i32);  // Convert usize to i32
        }
        
        let mut min_sum = i32::MAX;
        let mut result = Vec::new();
        
        // Iterate over list2 and check for common words
        for (j, word) in list2.iter().enumerate() {
            if let Some(&i) = index_map.get(word) {
                let sum = i + j as i32;  // Convert usize to i32
                if sum < min_sum {
                    min_sum = sum;
                    result.clear();
                    result.push(word.clone());
                } else if sum == min_sum {
                    result.push(word.clone());
                }
            }
        }
        
        result
    }
}
