/*
3439. Reschedule Meetings for Maximum Free Time I

You are given an integer eventTime denoting the duration of an event, where the event occurs from time t = 0 to time t = eventTime.

You are also given two integer arrays startTime and endTime, each of length n. These represent the start and end time of n non-overlapping meetings, where the ith meeting occurs during the time [startTime[i], endTime[i]].

You can reschedule at most k meetings by moving their start time while maintaining the same duration, to maximize the longest continuous period of free time during the event.

The relative order of all the meetings should stay the same and they should remain non-overlapping.

Return the maximum amount of free time possible after rearranging the meetings.

Note that the meetings can not be rescheduled to a time outside the event.
*/
impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        fn get_gaps(event_time: i32, start_time: &Vec<i32>, end_time: &Vec<i32>) -> Vec<i32> {
            let mut gaps = Vec::new();
            gaps.push(start_time[0]);
            for i in 1..start_time.len() {
                gaps.push(start_time[i] - end_time[i - 1]);
            }
            gaps.push(event_time - end_time[end_time.len() - 1]);
            gaps
        }

        let gaps = get_gaps(event_time, &start_time, &end_time);
        let k = k as usize;

        if gaps.len() < k + 1 {
            return gaps.iter().sum(); // not enough gaps to form a window
        }

        let mut window_sum: i32 = gaps.iter().take(k + 1).sum();
        let mut ans = window_sum;

        for i in (k + 1)..gaps.len() {
            window_sum += gaps[i] - gaps[i - k - 1];
            ans = ans.max(window_sum);
        }

        ans
    }
}
