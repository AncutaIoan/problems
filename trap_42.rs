use std::cmp;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        fn get_max_l_r(height: &[i32], current_index: usize) -> i32 {
            let mut max_l = height[current_index];
            let mut max_r = height[current_index];

            let mut l = current_index;
            while l > 0 {
                l -= 1;
                if height[l] > max_l {
                    max_l = height[l];
                }
            }

            let mut r = current_index + 1;
            while r < height.len() {
                if height[r] > max_r {
                    max_r = height[r];
                }
                r += 1;
            }

            cmp::min(max_l, max_r)
        }

        let mut total_water = 0;

        for i in 1..height.len() - 1 {
            let water_level = get_max_l_r(&height, i);
            if water_level > height[i] {
                total_water += water_level - height[i];
            }
        }

        total_water
    }
}
