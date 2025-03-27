use std::cmp::max;

impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        // Sort events by their end time
        events.sort_by_key(|event| event[1]);

        let num_events = events.len();
        let k = k as usize;
        let mut dp = vec![vec![0; k + 1]; num_events + 1];

        // Function to perform binary search and find last non-overlapping event
        fn find_last_event(events: &Vec<Vec<i32>>, index: usize) -> usize {
            let start_time = events[index][0];
            let mut low = 0;
            let mut high = index;

            while low < high {
                let mid = (low + high) / 2;
                if events[mid][1] < start_time {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }

            if low > 0 && events[low - 1][1] < start_time {
                return low;
            }
            0
        }

        // DP processing
        for i in 1..=num_events {
            let (start_time, _end_time, value) = (events[i - 1][0], events[i - 1][1], events[i - 1][2]);
            let last_event_index = find_last_event(&events, i - 1);

            for j in 1..=k {
                // Either skip the current event or take it and add its value
                dp[i][j] = max(dp[i - 1][j], dp[last_event_index][j - 1] + value);
            }
        }

        dp[num_events][k]
    }
}
