use std::collections::{HashSet, VecDeque};

impl Solution {
    const MOD: i64 = 1_000_000_007;
    
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut k = k as i64;
        let mut arr: Vec<(usize, usize, i32)> = (0..n)
            .map(|i| (i, Self::prime_factors(nums[i]), nums[i]))
            .collect();
        
        let mut left = vec![-1; n];
        let mut right = vec![n as i32; n];
        let mut stk: VecDeque<usize> = VecDeque::new();
        
        for (i, f, _) in &arr {
            while let Some(&top) = stk.back() {
                if arr[top].1 < *f {
                    stk.pop_back();
                } else {
                    break;
                }
            }
            if let Some(&top) = stk.back() {
                left[*i] = top as i32;
            }
            stk.push_back(*i);
        }
        
        stk.clear();
        for i in (0..n).rev() {
            let f = arr[i].1;
            while let Some(&top) = stk.back() {
                if arr[top].1 <= f {
                    stk.pop_back();
                } else {
                    break;
                }
            }
            if let Some(&top) = stk.back() {
                right[i] = top as i32;
            }
            stk.push_back(i);
        }
        
        arr.sort_by(|a, b| b.2.cmp(&a.2));
        let mut ans: i64 = 1;
        
        for (i, _, x) in arr {
            let l = left[i] as i64;
            let r = right[i] as i64;
            let cnt = (i as i64 - l) * (r - i as i64);
            
            if cnt <= k {
                ans = ans * Self::qpow(x as i64, cnt) % Self::MOD;
                k -= cnt;
            } else {
                ans = ans * Self::qpow(x as i64, k) % Self::MOD;
                break;
            }
        }
        ans as i32
    }
    
    fn prime_factors(mut n: i32) -> usize {
        let mut factors = HashSet::new();
        let mut i = 2;
        while i * i <= n {
            while n % i == 0 {
                factors.insert(i);
                n /= i;
            }
            i += 1;
        }
        if n > 1 {
            factors.insert(n);
        }
        factors.len()
    }
    
    fn qpow(mut a: i64, mut n: i64) -> i64 {
        let mut ans = 1;
        while n > 0 {
            if n & 1 == 1 {
                ans = ans * a % Self::MOD;
            }
            a = a * a % Self::MOD;
            n >>= 1;
        }
        ans
    }
}
