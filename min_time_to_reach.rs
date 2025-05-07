/*
3341. Find Minimum Time to Reach Last Room I
There is a dungeon with n x m rooms arranged as a grid.

You are given a 2D array moveTime of size n x m, where moveTime[i][j] represents the minimum time in seconds when you can start moving to that room. You start from the room (0, 0) at time t = 0 and can move to an adjacent room. Moving between adjacent rooms takes exactly one second.

Return the minimum time to reach the room (n - 1, m - 1).

Two rooms are adjacent if they share a common wall, either horizontally or vertically.

 

Example 1:

Input: moveTime = [[0,4],[4,4]]

Output: 6

Explanation:

The minimum time required is 6 seconds.

At time t == 4, move from room (0, 0) to room (1, 0) in one second.
At time t == 5, move from room (1, 0) to room (1, 1) in one second.
Example 2:

Input: moveTime = [[0,0,0],[0,0,0]]

Output: 3

Explanation:

The minimum time required is 3 seconds.

At time t == 0, move from room (0, 0) to room (1, 0) in one second.
At time t == 1, move from room (1, 0) to room (1, 1) in one second.
At time t == 2, move from room (1, 1) to room (1, 2) in one second.
Example 3:

Input: moveTime = [[0,1],[1,2]]

Output: 3

 

Constraints:

2 <= n == moveTime.length <= 50
2 <= m == moveTime[i].length <= 50
0 <= moveTime[i][j] <= 109

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let rows = move_time.len();
        let cols = move_time[0].len();

        let mut min_heap = BinaryHeap::new();
        min_heap.push(Reverse((0, 0, 0)));

        let mut arrival_time = vec![vec![i32::MAX; cols]; rows];
        arrival_time[0][0] = 0;

        while let Some(Reverse((curr_time, x, y))) = min_heap.pop() {
            if x == rows - 1 && y == cols - 1 {
                return curr_time;
            }

            for &(dx, dy) in &Self::DIRS {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;

                if new_x >= 0 && new_x < rows as i32 && new_y >= 0 && new_y < cols as i32 {
                    let new_x = new_x as usize;
                    let new_y = new_y as usize;

                    let wait_time = 0.max(move_time[new_x][new_y] - curr_time);
                    let new_arrival_time = curr_time + 1 + wait_time;

                    if new_arrival_time < arrival_time[new_x][new_y] {
                        arrival_time[new_x][new_y] = new_arrival_time;
                        min_heap.push(Reverse((new_arrival_time, new_x, new_y)));
                    }
                }
            }
        }

        -1
    }
}

*/

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let n = move_time.len();
        let m = move_time[0].len();
        
        let mut dist = vec![vec![i32::MAX; m]; n]; // distance to each cell, initialized to max value
        
        let dx = [-1, 1, 0, 0]; // Directions: up, down, left, right
        let dy = [0, 0, -1, 1];
        
        // Min-heap to process the grid in the order of earliest reachable time
        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0, 0, 0))); // {time, x, y}, start at (0, 0) with time 0
        
        // We need a mutable copy of move_time, as we'll be marking rooms as visited
        let mut move_time = move_time.clone();

        while let Some(Reverse((time, x, y))) = pq.pop() {
            // Mark the node as visited (set its moveTime to -1)
            move_time[x][y] = -1;
            
            // If we've reached the bottom-right corner, return the current time
            if x == n - 1 && y == m - 1 {
                return time;
            }

            // Traverse the 4 adjacent directions
            for d in 0..4 {
                let nx = x as isize + dx[d];
                let ny = y as isize + dy[d];

                // Check if the new coordinates are valid and unvisited
                if nx < 0 || ny < 0 || nx >= n as isize || ny >= m as isize || move_time[nx as usize][ny as usize] == -1 {
                    continue;
                }

                // Determine the time to reach the new cell
                let time_n = time.max(move_time[nx as usize][ny as usize]);
                
                // If we find a better time to reach (nx, ny), update and push to the queue
                if dist[nx as usize][ny as usize] > time_n + 1 {
                    dist[nx as usize][ny as usize] = time_n + 1;
                    pq.push(Reverse((time_n + 1, nx as usize, ny as usize)));
                }
            }
        }

        // Redundant return, as we would already return earlier when we reach the last cell
        dist[n - 1][m - 1]
    }
}
