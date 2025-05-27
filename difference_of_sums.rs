impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let total_sum = n * (n + 1) / 2;
        let k = n / m;
        let sum_divisible_by_m = m * k * (k + 1) / 2;
        total_sum - 2 * sum_divisible_by_m
    }
}
