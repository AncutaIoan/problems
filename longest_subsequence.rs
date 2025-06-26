impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut longest_length = 0;  // Length of the longest valid subsequence
        let mut decimal_value = 0;   // Decimal value of the considered subsequence

        // Iterate from the end (least significant bit first)
        for i in (0..chars.len()).rev() {
            if chars[i] == '0' {
                longest_length += 1; // Always safe to include '0'
            } else if longest_length < 30 {
                // Check if adding this '1' keeps the value ≤ k
                if decimal_value | (1 << longest_length) <= k {
                    decimal_value |= 1 << longest_length;
                    longest_length += 1;
                }
            }
        }

        longest_length
    }

}
