/*

3342. Find Minimum Time to Reach Last Room II


There is a dungeon with n x m rooms arranged as a grid.

You are given a 2D array moveTime of size n x m, where moveTime[i][j] represents the minimum time in seconds when you can start moving to that room. You start from the room (0, 0) at time t = 0 and can move to an adjacent room. Moving between adjacent rooms takes one second for one move and two seconds for the next, alternating between the two.

Return the minimum time to reach the room (n - 1, m - 1).

Two rooms are adjacent if they share a common wall, either horizontally or vertically.

Constraints:

2 <= n == moveTime.length <= 750
2 <= m == moveTime[i].length <= 750
0 <= moveTime[i][j] <= 109


Example 1:

Input: moveTime = [[0,4],[4,4]]

Output: 7

Explanation:

The minimum time required is 7 seconds.

At time t == 4, move from room (0, 0) to room (1, 0) in one second.
At time t == 5, move from room (1, 0) to room (1, 1) in two seconds.

*/





use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let n = move_time.len();
        let m = move_time[0].len();

        let mut dist = vec![vec![vec![i32::MAX; 2]; m]; n];
        let mut heap = BinaryHeap::new();

        dist[0][0][0] = 0;
        heap.push(Reverse((0, 0, 0, 0))); // (time, i, j, parity)

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some(Reverse((time, i, j, parity))) = heap.pop() {
            if i == n - 1 && j == m - 1 {
                return time;
            }

            if time > dist[i][j][parity] {
                continue;
            }

            for (dx, dy) in directions.iter() {
                let ni = i as i32 + dx;
                let nj = j as i32 + dy;
                //bounds check
                if ni < 0 || nj < 0 || ni >= n as i32 || nj >= m as i32 {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;

                let move_cost = if parity == 0 { 1 } else { 2 };

                let wait_time = move_time[ni][nj].max(time);
                let next_time = wait_time + move_cost;
                //this is only check for line if expression above
                let next_parity = 1 - parity;

                // find smallest
                if next_time < dist[ni][nj][next_parity] {
                    dist[ni][nj][next_parity] = next_time;
                    heap.push(Reverse((next_time, ni, nj, next_parity)));
                }
            }
        }

        dist[n - 1][m - 1][0].min(dist[n - 1][m - 1][1])
    }
}
