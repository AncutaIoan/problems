/*
3202. Find the Maximum Length of Valid Subsequence II
You are given an integer array nums and a positive integer k.
A subsequence sub of nums with length x is called valid if it satisfies:

(sub[0] + sub[1]) % k == (sub[1] + sub[2]) % k == ... == (sub[x - 2] + sub[x - 1]) % k.
Return the length of the longest valid subsequence of nums.


*/
impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut subarray_lengths = vec![vec![0; k]; k];
        let mut max_length = 0;

        for &num in &nums {
            let mod_value = ((num % k as i32 + k as i32) % k as i32) as usize;

            // Collect updates to apply after the loop
            let mut updates = vec![];

            for j in 0..k {
                let required_mod = (j + k - mod_value) % k;
                let new_len = subarray_lengths[required_mod][mod_value] + 1;

                updates.push((mod_value, required_mod, new_len));
                max_length = max_length.max(new_len);
            }

            // Apply updates
            for (mod_val, req_mod, new_len) in updates {
                subarray_lengths[mod_val][req_mod] = subarray_lengths[mod_val][req_mod].max(new_len);
            }
        }

        max_length
    }
}
