use std::collections::HashMap;

impl Solution {
    fn last_positions(s: &str) -> HashMap<char, usize> {
        let mut map = HashMap::new();
    
        for (i, c) in s.chars().enumerate() {
            map.insert(c, i);
        }
        
        map
    }

    pub fn partition_labels(s: String) -> Vec<i32> {
        let letter_indexes = Solution::last_positions(&s);

        let mut partitions = Vec::new();
        let mut max_index = 0;
        let mut anchor = 0;

        for (i, c) in s.chars().enumerate() {
            if let Some(&last_pos) = letter_indexes.get(&c) {
                max_index = max_index.max(last_pos);
            }

            if max_index == i {
                partitions.push((i - anchor + 1) as i32);
                anchor = i + 1;
            }
        }

        partitions
    }
}
