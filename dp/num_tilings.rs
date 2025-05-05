impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = n as usize;

        if n == 0 {
            return 1;
        }

        let mut dp = vec![0i64; n + 1];
        dp[0] = 1;
        if n >= 1 {
            dp[1] = 1;
        }
        if n >= 2 {
            dp[2] = 2;
        }

        for i in 3..=n {
            dp[i] = (2 * dp[i - 1] + dp[i - 3]) % MOD;
        }

        dp[n] as i32
    }
}
