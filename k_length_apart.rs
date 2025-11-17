impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut prev_one_index: Option<usize> = None;

        for (i, &num) in nums.iter().enumerate() {
            if num == 1 {
                if let Some(prev) = prev_one_index {
                    if i - prev - 1 < k as usize {
                        return false;
                    }
                }
                prev_one_index = Some(i);
            }
        }
        true
    }
}
