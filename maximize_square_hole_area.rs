impl Solution {
    pub fn maximize_square_hole_area(
        _n: i32,
        _m: i32,
        mut h_bars: Vec<i32>,
        mut v_bars: Vec<i32>,
    ) -> i32 {
        fn longest_consecutive(bars: &mut Vec<i32>) -> i32 {
            if bars.is_empty() {
                return 0;
            }

            bars.sort_unstable();
            let mut max_run = 1;
            let mut current_run = 1;

            for i in 1..bars.len() {
                if bars[i] == bars[i - 1] + 1 {
                    current_run += 1;
                    max_run = max_run.max(current_run);
                } else {
                    current_run = 1;
                }
            }

            max_run
        }

        let max_h = longest_consecutive(&mut h_bars) + 1;
        let max_v = longest_consecutive(&mut v_bars) + 1;

        let side = max_h.min(max_v);
        side * side
    }
}
