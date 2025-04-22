impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = n as usize;
        let max_value = max_value as usize;

        // Step 1: Precompute factorials and inverse factorials
        let mut fact = vec![1i64; n + 1];
        let mut inv_fact = vec![1i64; n + 1];

        for i in 1..=n {
            fact[i] = fact[i - 1] * i as i64 % MOD;
        }

        inv_fact[n] = modinv(fact[n], MOD);
        for i in (1..n).rev() {
            inv_fact[i] = inv_fact[i + 1] * (i as i64 + 1) % MOD;
        }

        let comb = |a: usize, b: usize| -> i64 {
            if b > a {
                0
            } else {
                fact[a] * inv_fact[b] % MOD * inv_fact[a - b] % MOD
            }
        };

        // Step 2: Build divisor chains with DP
        let max_len = 14; // log2(10^4) ≈ 14, max chain length
        let mut dp = vec![vec![0i64; max_len + 1]; max_value + 1];

        for i in 1..=max_value {
            dp[i][1] = 1;
        }

        // For each length k, build using divisors of j
        for k in 2..=max_len {
            for j in 1..=max_value {
                let mut m = j * 2;
                while m <= max_value {
                    dp[m][k] = (dp[m][k] + dp[j][k - 1]) % MOD;
                    m += j;
                }
            }
        }

        // Step 3: Sum all possible sequences
        let mut result = 0;
        for x in 1..=max_value {
            for k in 1..=max_len {
                if dp[x][k] > 0 && k <= n {
                    result = (result + dp[x][k] * comb(n - 1, k - 1) % MOD) % MOD;
                }
            }
        }

        result as i32
    }
}

// Modular inverse using Fermat's Little Theorem
fn modinv(x: i64, m: i64) -> i64 {
    modpow(x, m - 2, m)
}

fn modpow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp /= 2;
    }
    result
}
