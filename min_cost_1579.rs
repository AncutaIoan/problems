/*
1578. Minimum Time to Make Rope Colorful

Alice has n balloons arranged on a rope. You are given a 0-indexed string colors where colors[i] is the color of the ith balloon.

Alice wants the rope to be colorful. She does not want two consecutive balloons to be of the same color, so she asks Bob for help. Bob can remove some balloons from the rope to make it colorful. You are given a 0-indexed integer array neededTime where neededTime[i] is the time (in seconds) that Bob needs to remove the ith balloon from the rope.

Return the minimum time Bob needs to make the rope colorful.
*/
impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let colors = colors.as_bytes(); // to work with chars efficiently
        let mut total_cost = 0;
        let mut i = 0;
        let n = colors.len();

        while i < n {
            let mut j = i;
            let mut group_sum = 0;
            let mut group_max = 0;

            // find the end of the current same-color group
            while j < n && colors[j] == colors[i] {
                group_sum += needed_time[j];
                group_max = group_max.max(needed_time[j]);
                j += 1;
            }

            // we keep one balloon (the one with max removal time)
            total_cost += group_sum - group_max;

            // move to next group
            i = j;
        }

        total_cost
    }
}
