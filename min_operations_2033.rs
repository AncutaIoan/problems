impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        fn median(vec: &mut Vec<i32>) -> i32 {
            vec.sort_unstable();
            let len = vec.len();
            vec[len / 2]
        }

        // Flatten the grid into a single vector
        let mut values: Vec<i32> = grid.into_iter().flatten().collect();

        // Find the median
        let median = median(&mut values);

        let mut count_oprs = 0;
        for element in values {
            // If the difference is not divisible by x, it's impossible
            if (element - median) % x != 0 {
                return -1;
            }
            count_oprs += (element - median).abs() / x;
        }

        count_oprs
    }
}
