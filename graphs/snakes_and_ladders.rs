/*
909. Snakes and Ladders

You are given an n x n integer matrix board where the cells are labeled from 1 to n2 in a Boustrophedon style starting from the bottom left of the board (i.e. board[n - 1][0]) and alternating direction each row.

You start on square 1 of the board. In each move, starting from square curr, do the following:

Choose a destination square next with a label in the range [curr + 1, min(curr + 6, n2)].
This choice simulates the result of a standard 6-sided die roll: i.e., there are always at most 6 destinations, regardless of the size of the board.
If next has a snake or ladder, you must move to the destination of that snake or ladder. Otherwise, you move to next.
The game ends when you reach the square n2.
A board square on row r and column c has a snake or ladder if board[r][c] != -1. The destination of that snake or ladder is board[r][c]. Squares 1 and n2 are not the starting points of any snake or ladder.

Note that you only take a snake or ladder at most once per dice roll. If the destination to a snake or ladder is the start of another snake or ladder, you do not follow the subsequent snake or ladder.

For example, suppose the board is [[-1,4],[-1,3]], and on the first move, your destination square is 2. You follow the ladder to square 3, but do not follow the subsequent ladder to 4.
Return the least number of dice rolls required to reach the square n2. If it is not possible to reach the square, return -1.

*/

use std::collections::VecDeque;
use std::collections::HashSet;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        
        // Helper function to map a square number to board coordinates.
        fn get_coords(square: i32, n: usize) -> (usize, usize) {
            let quot = (square - 1) / n as i32;
            let rem = (square - 1) % n as i32;
            let row = n - 1 - quot as usize;
            let col = if quot % 2 == 0 { rem as usize } else { n - 1 - rem as usize };
            (row, col)
        }
        
        let mut visited = vec![false; n * n + 1];
        let mut queue = VecDeque::new();
        queue.push_back((1, 0)); // (square, moves)
        visited[1] = true;

        while let Some((square, moves)) = queue.pop_front() {
            for next in square+1..=std::cmp::min(square+6, (n * n) as i32) {
                let (r, c) = get_coords(next, n);
                let dest = if board[r][c] == -1 { next } else { board[r][c] };
                if dest == (n * n) as i32 {
                    return moves + 1;
                }
                if !visited[dest as usize] {
                    visited[dest as usize] = true;
                    queue.push_back((dest, moves + 1));
                }
            }
        }

        -1
    }
}
