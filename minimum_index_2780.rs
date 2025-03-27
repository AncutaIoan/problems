use std::collections::HashMap;

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let mut freq = HashMap::new();

        for &i in &nums {
            *freq.entry(i).or_insert(0) += 1;
        }

        let max_key = *freq.iter().max_by_key(|&(_, v)| v).unwrap().0;
        let mut count_1 = 0;
        let mut count_2 = freq[&max_key];

        for i in 0..nums.len() {
            if nums[i] == max_key {
                count_1 += 1;
                count_2 -= 1;
            }

            if count_1 * 2 > (i + 1) as i32 && count_2 * 2 > (nums.len() - (i + 1)) as i32 {
                return i as i32;
            }
        }

        -1
    }
}
