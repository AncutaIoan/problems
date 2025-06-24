impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        let mut count = 0;
        let mut sum = 0;
        let mut len = 1;

        while count < n {
            // Generate palindromes of length `len`
            let start = 10_i64.pow(((len - 1) / 2) as u32);
            let end = 10_i64.pow(((len + 1) / 2) as u32);

            for half in start..end {
                let pal = Self::create_palindrome(half, len % 2 == 1);
                if Self::is_k_palindrome(pal, k) {
                    sum += pal;
                    count += 1;
                    if count == n {
                        break;
                    }
                }
            }

            len += 1;
        }

        sum
    }

    fn create_palindrome(half: i64, odd_length: bool) -> i64 {
        let mut n = half;
        let mut rev = if odd_length { n / 10 } else { n };

        while rev > 0 {
            n = n * 10 + rev % 10;
            rev /= 10;
        }

        n
    }

    fn is_k_palindrome(mut num: i64, k: i32) -> bool {
        let mut digits = Vec::new();

        while num > 0 {
            digits.push((num % k as i64) as i32);
            num /= k as i64;
        }

        let len = digits.len();
        for i in 0..len / 2 {
            if digits[i] != digits[len - 1 - i] {
                return false;
            }
        }

        true
    }
}
