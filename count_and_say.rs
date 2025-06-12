impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }
        let mut result = "1".to_string();
        for _ in 2..=n {
            let mut current = String::new();
            let chars: Vec<char> = result.chars().collect();
            let mut count = 1;

            for i in 1..chars.len() {
                if chars[i] == chars[i - 1] {
                    count += 1;
                } else {
                    current.push_str(&format!("{}{}", count, chars[i - 1]));
                    count = 1;
                }
            }

            // Append the last group
            current.push_str(&format!("{}{}", count, chars[chars.len() - 1]));
            result = current;
        }

        result
    }
}
