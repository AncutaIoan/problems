/*

1922. Count Good Numbers

A digit string is good if the digits (0-indexed) at even indices are even and the digits at odd indices are prime (2, 3, 5, or 7).

For example, "2582" is good because the digits (2 and 8) at even positions are even and the digits (5 and 2) at odd positions are prime. However, "3245" is not good because 3 is at an even index but is not even.
Given an integer n, return the total number of good digit strings of length n. Since the answer may be large, return it modulo 109 + 7.

A digit string is a string consisting of digits 0 through 9 that may contain leading zeros.

*/

impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        const MOD: i64 = 1_000_000_007;

        fn mod_pow(mut base: i64, mut exp: i64, modulo: i64) -> i64 {
            let mut result = 1;
            base %= modulo;
            while exp > 0 {
                if exp % 2 == 1 {
                    result = result * base % modulo;
                }
                base = base * base % modulo;
                exp /= 2;
            }
            result
        }

        let even_positions = (n + 1) / 2;
        let odd_positions = n / 2;

        let count = mod_pow(5, even_positions, MOD) * mod_pow(4, odd_positions, MOD) % MOD;
        count as i32
    }
}
