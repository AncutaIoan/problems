/*
1353. Maximum Number of Events That Can Be Attended

You are given an array of events where events[i] = [startDayi, endDayi]. Every event i starts at startDayi and ends at endDayi.

You can attend an event i at any day d where startTimei <= d <= endTimei. You can only attend one event at any time d.

Return the maximum number of events you can attend.
*/
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
        // Sort the events by start day
        events.sort_by_key(|e| e[0]);
        
        let mut min_heap = BinaryHeap::new(); // Will store end times (min-heap using Reverse)
        let mut day = 1;
        let mut i = 0;
        let mut res = 0;
        let n = events.len();

        // Find the maximum last day
        let max_day = events.iter().map(|e| e[1]).max().unwrap_or(0);

        while day <= max_day {
            // Push all events starting today into the heap
            while i < n && events[i][0] == day {
                min_heap.push(Reverse(events[i][1]));
                i += 1;
            }

            // Remove all events that have already expired
            while let Some(&Reverse(end)) = min_heap.peek() {
                if end < day {
                    min_heap.pop();
                } else {
                    break;
                }
            }

            // Attend the event with the earliest end time
            if let Some(_) = min_heap.pop() {
                res += 1;
            }

            day += 1;
        }

        res
    }
}
