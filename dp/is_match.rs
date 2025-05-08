use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];

        dp[0][0] = true;

        for i in 0..=s.len() {
            for j in 1..=p.len() {
                if p.chars().nth(j - 1) == Some('*') {
                    dp[i][j] = dp[i][j - 2];
                    if i > 0 && (p.chars().nth(j - 2) == Some('.') || p.chars().nth(j - 2) == s.chars().nth(i - 1)) {
                        dp[i][j] |= dp[i - 1][j];
                    }
                } else {
                    if i > 0 && (p.chars().nth(j - 1) == Some('.') || p.chars().nth(j - 1) == s.chars().nth(i - 1)) {
                        dp[i][j] = dp[i - 1][j - 1];
                    }
                }
            }
        }

        dp[s.len()][p.len()]
    }
}
/*
10. Regular Expression Matching

Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:

'.' Matches any single character.​​​​
'*' Matches zero or more of the preceding element.
The matching should cover the entire input string (not partial).

 

Example 1:

Input: s = "aa", p = "a"
Output: false
Explanation: "a" does not match the entire string "aa".
Example 2:

Input: s = "aa", p = "a*"
Output: true
Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
Example 3:

Input: s = "ab", p = ".*"
Output: true
Explanation: ".*" means "zero or more (*) of any character (.)".
class Solution {
    public boolean isMatch(String s, String p) {
        boolean[][] dp = new boolean[s.length() + 1][p.length() + 1];

        dp[0][0] = true;
        Set<Character> checkP = Set.of('*', '.');

        // for(int i = 1; i < s.length() + 1; i++) {
        //     for(int j = 1; j< p.length() + 1; j++) {
        //         if (!checkP.contains(p.charAt(j - 1)) && p.charAt(j - 1) == s.charAt(i - 1) && dp[i-1][j - 1]) {
        //             dp[i][j] = true;
        //         } else if (p.charAt(j-1) == '.' && dp[i-1][j-1]) { 
        //             dp[i][j] = true;
        //         } else if (p.charAt(j - 1) == '*' && dp[i][j - 2]) { 
        //             dp[i][j] = true; 
        //         } else if (p.charAt(j - 1) == '*' && 
        //                    (p.charAt(j - 2) == s.charAt(i - 1) || p.charAt(j - 2) == '.') && 
        //                    dp[i - 1][j]) {
        //             dp[i][j] = true; 
        //         }
        //     }
        // }
        for (int i = 0; i <= s.length(); i++) {
            for (int j = 1; j <= p.length(); j++) {
                if (p.charAt(j - 1) == '*') {
                    // Check the position without the '*' pair (reduce pattern by 2)
                    dp[i][j] = dp[i][j - 2];
                    // If text character matches pattern character before '*' or if it's a '.'
                    if (i > 0 && (p.charAt(j - 2) == '.' || p.charAt(j - 2) == s.charAt(i - 1))) {
                        // 'OR' with the position above to see if any prev occurrences match
                        dp[i][j] |= dp[i - 1][j];
                    }
                } else {
                    // For '.' or exact match, current dp position is based on the prev diagonal position
                    if (i > 0 && (p.charAt(j - 1) == '.' || p.charAt(j - 1) == s.charAt(i - 1))) {
                        dp[i][j] = dp[i - 1][j - 1];
                    }
                }
            }
        }

        return dp[s.length()][p.length()];
    }
}*/
