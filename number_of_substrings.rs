/*
3234. Count the Number of Substrings With Dominant Ones

You are given a binary string s.

Return the number of substrings with dominant ones.

A string has dominant ones if the number of ones in the string is greater than or equal to the square of the number of zeros in the string.
*/
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let n = s.len();
        let bytes = s.as_bytes();
        let mut prefix_ones = vec![0; n + 1];
        let mut prefix_zeros = vec![0; n + 1];

        for i in 0..n {
            prefix_ones[i + 1] = prefix_ones[i] + (bytes[i] == b'1') as i32;
            prefix_zeros[i + 1] = prefix_zeros[i] + (bytes[i] == b'0') as i32;
        }

        let mut ans: i64 = 0;
        let limit = 450; // sqrt(2e5) ≈ 447

        for l in 0..n {
            let mut low_zero_count = 0;

            // Growing r until zeros exceed limit
            for r in l..n {
                if bytes[r] == b'0' {
                    low_zero_count += 1;
                    if low_zero_count > limit {
                        break;
                    }
                }

                let ones = prefix_ones[r + 1] - prefix_ones[l];
                let zeros = low_zero_count;

                if ones >= zeros * zeros {
                    ans += 1;
                }
            }
        }

        ans as i32
    }
}
