impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut min_prefix: i64 = 0;
        let mut max_prefix: i64 = 0;
        let mut current: i64 = 0;

        for &diff in &differences {
            current += diff as i64;
            min_prefix = min_prefix.min(current);
            max_prefix = max_prefix.max(current);
        }

        let min_start = lower as i64 - min_prefix;
        let max_start = upper as i64 - max_prefix;

        if max_start < min_start {
            0
        } else {
            (max_start - min_start + 1) as i32
        }
    }
}
