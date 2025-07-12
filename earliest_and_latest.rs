/*
1900. The Earliest and Latest Rounds Where Players Compete

There is a tournament where n players are participating. The players are standing in a single row and are numbered from 1 to n based on their initial standing position (player 1 is the first player in the row, player 2 is the second player in the row, etc.).

The tournament consists of multiple rounds (starting from round number 1). In each round, the ith player from the front of the row competes against the ith player from the end of the row, and the winner advances to the next round. When the number of players is odd for the current round, the player in the middle automatically advances to the next round.

For example, if the row consists of players 1, 2, 4, 6, 7
Player 1 competes against player 7.
Player 2 competes against player 6.
Player 4 automatically advances to the next round.
After each round is over, the winners are lined back up in the row based on the original ordering assigned to them initially (ascending order).

The players numbered firstPlayer and secondPlayer are the best in the tournament. They can win against any other player before they compete against each other. If any two other players compete against each other, either of them might win, and thus you may choose the outcome of this round.

Given the integers n, firstPlayer, and secondPlayer, return an integer array containing two values, the earliest possible round number and the latest possible round number in which these two players will compete against each other, respectively.
*/ 
use std::cmp::{min, max};

impl Solution {
    pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
        let mut mem = vec![vec![vec![(0, 0); (n + 1) as usize]; (n + 1) as usize]; (n + 1) as usize];
        let (a, b) = Self::solve(first_player as usize, (n - second_player + 1) as usize, n as usize, &mut mem);
        vec![a as i32, b as i32]
    }

    fn solve(
        l: usize,
        r: usize,
        k: usize,
        mem: &mut Vec<Vec<Vec<(i32, i32)>>>,
    ) -> (i32, i32) {
        if l == r {
            return (1, 1);
        }
        let (l, r) = if l > r { (r, l) } else { (l, r) };

        if mem[l][r][k] != (0, 0) {
            return mem[l][r][k];
        }

        let mut a = i32::MAX;
        let mut b = i32::MIN;

        for i in 1..=l {
            for j in (l - i + 1)..=(r - i) {
                if i + j > (k + 1) / 2 || i + j < l + r - k / 2 {
                    continue;
                }
                let (x, y) = Self::solve(i, j, (k + 1) / 2, mem);
                a = min(a, x + 1);
                b = max(b, y + 1);
            }
        }

        mem[l][r][k] = (a, b);
        (a, b)
    }
}
