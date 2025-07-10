/*
3440. Reschedule Meetings for Maximum Free Time II

You are given an integer eventTime denoting the duration of an event. You are also given two integer arrays startTime and endTime, each of length n.

These represent the start and end times of n non-overlapping meetings that occur during the event between time t = 0 and time t = eventTime, where the ith meeting occurs during the time [startTime[i], endTime[i]].

You can reschedule at most one meeting by moving its start time while maintaining the same duration, such that the meetings remain non-overlapping, to maximize the longest continuous period of free time during the event.

Return the maximum amount of free time possible after rearranging the meetings.

Note that the meetings can not be rescheduled to a time outside the event and they should remain non-overlapping.

Note: In this version, it is valid for the relative ordering of the meetings to change after rescheduling one meeting.
*/
impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        if n == 0 {
            return event_time;
        }

        let mut max_free_time = 0;

        // Calculate left gaps
        let mut left_gaps = vec![0; n];
        left_gaps[0] = start_time[0];
        for i in 1..n {
            left_gaps[i] = left_gaps[i - 1].max(start_time[i] - end_time[i - 1]);
        }

        // Calculate right gaps
        let mut right_gaps = vec![0; n];
        right_gaps[n - 1] = event_time - end_time[n - 1];
        for i in (0..n - 1).rev() {
            right_gaps[i] = right_gaps[i + 1].max(start_time[i + 1] - end_time[i]);
        }

        // Compute max free time
        for i in 0..n {
            let left_gap = if i == 0 {
                left_gaps[i]
            } else {
                start_time[i] - end_time[i - 1]
            };

            let right_gap = if i == n - 1 {
                right_gaps[i]
            } else {
                start_time[i + 1] - end_time[i]
            };

            let interval = if (i != 0 && left_gaps[i - 1] >= (end_time[i] - start_time[i]))
                || (i != n - 1 && right_gaps[i + 1] >= (end_time[i] - start_time[i]))
            {
                end_time[i] - start_time[i]
            } else {
                0
            };

            max_free_time = max_free_time.max(left_gap + interval + right_gap);
        }

        max_free_time
    }
}
