impl Solution {
    pub fn clear_stars(s: String) -> String {
        let mut stack = Vec::new();

        for c in s.chars() {
            if c == '*' {
                // Remove the smallest character to the left (top of stack)
                if let Some(_) = stack.pop() {}
            } else {
                stack.push(c);
            }
        }

        stack.into_iter().collect()
    }
}
