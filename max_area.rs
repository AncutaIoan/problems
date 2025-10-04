impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
    /*
            11. Container With Most Water

You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

Find two lines that together with the x-axis form a container, such that the container contains the most water.

Return the maximum amount of water a container can store.

Notice that you may not slant the container.
    */

        let mut s: usize = 0;
        let mut e: usize = height.len() - 1;
        let mut result = 0;

        while s < e {
            let width = e - s;
            let current = width as i32 * height[s].min(height[e]);

            result = result.max(current);

            if height[s] < height[e] {
                s += 1;
            } else {
                e -= 1;
            }
        }

        result
    }
}
