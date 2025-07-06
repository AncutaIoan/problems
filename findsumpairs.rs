/*
1865. Finding Pairs With a Certain Sum

You are given two integer arrays nums1 and nums2. You are tasked to implement a data structure that supports queries of two types:

Add a positive integer to an element of a given index in the array nums2.
Count the number of pairs (i, j) such that nums1[i] + nums2[j] equals a given value (0 <= i < nums1.length and 0 <= j < nums2.length).
Implement the FindSumPairs class:

FindSumPairs(int[] nums1, int[] nums2) Initializes the FindSumPairs object with two integer arrays nums1 and nums2.
void add(int index, int val) Adds val to nums2[index], i.e., apply nums2[index] += val.
int count(int tot) Returns the number of pairs (i, j) such that nums1[i] + nums2[j] == tot.
*/
use std::collections::HashMap;

struct FindSumPairs {
    nums_one: Vec<i32>,
    nums_two: Vec<i32>,
    freq_map: HashMap<i32, i32>, // stores frequency of nums_two values
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut freq_map = HashMap::new();
        for &num in &nums2 {
            *freq_map.entry(num).or_insert(0) += 1;
        }

        FindSumPairs {
            nums_one: nums1,
            nums_two: nums2,
            freq_map,
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        if (index as usize) >= self.nums_two.len() {
            return;
        }

        let old_val = self.nums_two[index as usize];
        let new_val = old_val + val;

        // Update nums_two
        self.nums_two[index as usize] = new_val;

        // Update frequency map
        *self.freq_map.entry(old_val).or_default() -= 1;
        if self.freq_map[&old_val] == 0 {
            self.freq_map.remove(&old_val);
        }
        *self.freq_map.entry(new_val).or_insert(0) += 1;
    }

    fn count(&self, tot: i32) -> i32 {
        let mut result = 0;

        for &num in &self.nums_one {
            let complement = tot - num;
            if let Some(&count) = self.freq_map.get(&complement) {
                result += count;
            }
        }

        result
    }
}

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * let obj = FindSumPairs::new(nums1, nums2);
 * obj.add(index, val);
 * let ret_2: i32 = obj.count(tot);
 */
