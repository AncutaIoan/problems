impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut ans = 0;
        let mut ones = 0;
        let chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() {
            if chars[i] == '1' {
                ones += 1;
            } else { // chars[i] == '0'
                if i + 1 == chars.len() || chars[i + 1] == '1' {
                    ans += ones;
                }
            }
        }

        ans
    }
}
