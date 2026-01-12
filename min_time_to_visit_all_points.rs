/*
1266. Minimum Time Visiting All Points

On a 2D plane, there are n points with integer coordinates points[i] = [xi, yi]. Return the minimum time in seconds to visit all the points in the order given by points.

You can move according to these rules:

In 1 second, you can either:
move vertically by one unit,
move horizontally by one unit, or
move diagonally sqrt(2) units (in other words, move one unit vertically then one unit horizontally in 1 second).
You have to visit the points in the same order as they appear in the array.
You are allowed to pass through points that appear later in the order, but these do not count as visits.
*/
use std::cmp;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 1 {
            return 0;
        }

        let mut start = &points[0];
        let mut current_target = 1;
        let mut total = 0;

        while current_target < points.len() {
            let rem_x = (points[current_target][0] - start[0]).abs();
            let rem_y = (points[current_target][1] - start[1]).abs();

            total += cmp::max(rem_x, rem_y);

            start = &points[current_target];
            current_target += 1;
        }

        total
    }
}
