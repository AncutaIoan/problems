impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        
        let mut stack = Vec::new();
        
        stack.push(-1);
        
        let mut max_len = 0;

        for (index, ch) in s.chars().enumerate() {
            if ch == '(' {
                stack.push(index as i32);
            } else {
                stack.pop();
                
                if let Some(&last) = stack.last() {
                    if last == -1 {
                        max_len = max_len.max(index as i32 + 1);
                    } else {
                        max_len = max_len.max(index as i32 - last);
                    }
                } else {
                    stack.push(index as i32);
                }
            }
        }
        
        max_len
    }
}
