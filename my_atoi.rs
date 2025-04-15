impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim_start(); // Step 1: Ignore leading whitespace
        let mut chars = s.chars().peekable();
        let mut sign = 1;

        // Step 2: Check for '+' or '-' sign
        if let Some(&c) = chars.peek() {
            if c == '-' {
                sign = -1;
                chars.next();
            } else if c == '+' {
                chars.next();
            }
        }

        let mut result: i64 = 0; // Use i64 to detect overflow before casting

        // Step 3: Parse digits and skip leading zeros
        while let Some(&c) = chars.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            let digit = (c as u8 - b'0') as i64;

            result = result * 10 + digit;

            // Step 4: Check for overflow
            if sign == 1 && result > i32::MAX as i64 {
                return i32::MAX;
            } else if sign == -1 && -result < i32::MIN as i64 {
                return i32::MIN;
            }

            chars.next();
        }

        // Step 5: Apply sign and return
        (sign * result as i64) as i32
    }
}
