impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort(); // Sort for binary search
        let mut count: i64 = 0;

        for i in 0..nums.len() {
            let left = Self::lower_bound(&nums, i + 1, lower - nums[i]);
            let right = Self::upper_bound(&nums, i + 1, upper - nums[i]);
            count += (right - left) as i64;
        }

        count
    }

    // First index where nums[idx] >= target
    fn lower_bound(nums: &[i32], start: usize, target: i32) -> usize {
        let mut l = start;
        let mut r = nums.len();
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m] < target {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }

    // First index where nums[idx] > target
    fn upper_bound(nums: &[i32], start: usize, target: i32) -> usize {
        let mut l = start;
        let mut r = nums.len();
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m] <= target {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }
}
