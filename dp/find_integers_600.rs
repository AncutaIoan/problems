/*
600. Non-negative Integers without Consecutive Ones

Given a positive integer n, return the number of the integers in the range [0, n] whose binary representations do not contain consecutive ones.

 

Example 1:

Input: n = 5
Output: 5
Explanation:
Here are the non-negative integers <= 5 with their corresponding binary representations:
0 : 0
1 : 1
2 : 10
3 : 11
4 : 100
5 : 101
Among them, only integer 3 disobeys the rule (two consecutive ones) and the other 5 satisfy the rule.


*/




impl Solution {
    // Array to store individual bits of the binary representation of n
    pub fn find_integers(n: i32) -> i32 {
        // Array to store the bits in reverse order
        let mut bits = vec![0; 33];
        let mut length = 0;

        // Convert n to binary and store in bits array
        let mut num = n;
        while num > 0 {
            length += 1;
            bits[length] = num & 1;
            num >>= 1;
        }

        // Memoization array to store intermediate results
        let mut memo = vec![vec![-1; 2]; 33];

        // Start the DFS process from the most significant bit
        Solution::dfs(length, 0, true, &bits, &mut memo)
    }

    // DFS function for solving the problem
    fn dfs(pos: usize, prev: i32, limit: bool, bits: &Vec<i32>, memo: &mut Vec<Vec<i32>>) -> i32 {
        // Base case: if position is 0, there is only one number which is 0.
        if pos == 0 {
            return 1;
        }

        // If the current state has been computed before, return the memoized result
        if !limit && memo[pos][prev as usize] != -1 {
            return memo[pos][prev as usize];
        }

        // Initialize the answer for this state
        let mut ans = 0;
        let upper_bound = if limit { bits[pos] } else { 1 };

        // Explore all possible values of the current bit (0 or 1)
        for i in 0..=upper_bound {
            // Skip if two consecutive 1's are encountered
            if prev == 1 && i == 1 {
                continue;
            }

            // Add result of the subproblem
            ans += Solution::dfs(pos - 1, i, limit && i == upper_bound, bits, memo);
        }

        // Memoize the result for the current state
        if !limit {
            memo[pos][prev as usize] = ans;
        }

        ans
    }
}
