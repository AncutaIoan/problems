/*
115. Distinct Subsequences

Given two strings s and t, return the number of distinct subsequences of s which equals t.

The test cases are generated so that the answer fits on a 32-bit signed integer.



*/

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s=s.into_bytes();
        let t=t.into_bytes();
        let mut answer: Vec<i32>=vec![0;t.len()+1];
        answer[0]=1;
        for x in s{
            for (i,y) in t.iter().copied().enumerate().rev(){
                if (x==y) {
                    answer[i+1]+=answer[i];
                }
            }
        }
        return answer[t.len()];
    }
    /*
    pub fn num_distinct(s: String, t: String) -> i32 {
        /*
          r a b b i t
        r             0
        a             0
        b             0
        b             0
        b             0 
        b             0
        i             0
        t         1 1 0
          0 0 0 0 0 0 0 
        */

        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
        let m = s_chars.len();
        let n = t_chars.len();
        
        // DP table where dp[i][j] represents the number of ways to form t[..j] using s[..i]
        let mut dp = vec![vec![0; n + 1]; m + 1];
        
        // Base case: An empty t (t[..0]) 
        // can always be formed from any prefix of s in exactly one way (by deleting all characters)
        for i in 0..=m {
            dp[i][0] = 1;
        }
        
        // Fill the DP table
        for i in 1..=m {
            for j in 1..=n {
                if s_chars[i - 1] == t_chars[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }
        
        dp[m][n]
    }
    */
}
