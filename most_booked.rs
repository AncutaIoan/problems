/*
2402. Meeting Rooms III

You are given an integer n. There are n rooms numbered from 0 to n - 1.

You are given a 2D integer array meetings where meetings[i] = [starti, endi] means that a meeting will be held during the half-closed time interval [starti, endi). All the values of starti are unique.

Meetings are allocated to rooms in the following manner:

Each meeting will take place in the unused room with the lowest number.
If there are no available rooms, the meeting will be delayed until a room becomes free. The delayed meeting should have the same duration as the original meeting.
When a room becomes unused, meetings that have an earlier original start time should be given the room.
Return the number of the room that held the most meetings. If there are multiple rooms, return the room with the lowest number.

A half-closed interval [a, b) is the interval between a and b including a and not including b.
*/

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        // Sort meetings by start time
        meetings.sort_by_key(|m| m[0]);

        let n = n as usize;

        // Min-heap of (end_time, room_index) for busy rooms
        let mut busy: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();

        // Min-heap of available room indices
        let mut idle: BinaryHeap<Reverse<usize>> = (0..n).map(Reverse).collect();

        let mut count = vec![0; n];

        for meeting in meetings {
            let start = meeting[0] as i64;
            let end = meeting[1] as i64;

            // Free up rooms whose meetings have ended by the start of this one
            while let Some(Reverse((free_time, room))) = busy.peek() {
                if *free_time <= start {
                    idle.push(Reverse(*room));
                    busy.pop();
                } else {
                    break;
                }
            }

            let room;
            if let Some(Reverse(i)) = idle.pop() {
                // Use an idle room
                room = i;
                busy.push(Reverse((end, room)));
            } else {
                // No idle room; wait for the soonest busy one
                let Reverse((free_time, i)) = busy.pop().unwrap();
                room = i;
                busy.push(Reverse((free_time + (end - start), room)));
            }

            count[room] += 1;
        }

        // Find the room with the most meetings
        let mut result = 0;
        for i in 0..n {
            if count[i] > count[result] {
                result = i;
            }
        }

        result as i32
    }
}
