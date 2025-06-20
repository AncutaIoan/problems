impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let direction_array: Vec<char> = s.chars().collect();

        // Try all direction pairs
        let max_se = Self::calculate_distance(&direction_array, k, 'S', 'E');
        let max_sw = Self::calculate_distance(&direction_array, k, 'S', 'W');
        let max_ne = Self::calculate_distance(&direction_array, k, 'N', 'E');
        let max_nw = Self::calculate_distance(&direction_array, k, 'N', 'W');

        *[max_se, max_sw, max_ne, max_nw].iter().max().unwrap()
    }

    fn calculate_distance(direction_array: &[char], k: i32, d1: char, d2: char) -> i32 {
        let mut current_max = 0;
        let mut max_distance = 0;
        let mut replacement_count = 0;

        for &dir in direction_array {
            if dir == d1 || dir == d2 {
                current_max += 1;
            } else if replacement_count < k {
                current_max += 1;
                replacement_count += 1;
            } else {
                current_max -= 1;
            }

            max_distance = max_distance.max(current_max);
        }

        max_distance
    }
}
