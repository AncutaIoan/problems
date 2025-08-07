/*
3363. Find the Maximum Number of Fruits Collected

There is a game dungeon comprised of n x n rooms arranged in a grid.

You are given a 2D array fruits of size n x n, where fruits[i][j] represents the number of fruits in the room (i, j). Three children will play in the game dungeon, with initial positions at the corner rooms (0, 0), (0, n - 1), and (n - 1, 0).

The children will make exactly n - 1 moves according to the following rules to reach the room (n - 1, n - 1):

The child starting from (0, 0) must move from their current room (i, j) to one of the rooms (i + 1, j + 1), (i + 1, j), and (i, j + 1) if the target room exists.
The child starting from (0, n - 1) must move from their current room (i, j) to one of the rooms (i + 1, j - 1), (i + 1, j), and (i + 1, j + 1) if the target room exists.
The child starting from (n - 1, 0) must move from their current room (i, j) to one of the rooms (i - 1, j + 1), (i, j + 1), and (i + 1, j + 1) if the target room exists.
When a child enters a room, they will collect all the fruits there. If two or more children enter the same room, only one child will collect the fruits, and the room will be emptied after they leave.

Return the maximum number of fruits the children can collect from the dungeon.
*/
impl Solution {
    pub fn max_collected_fruits(fruits: Vec<Vec<i32>>) -> i32 {
        let n = fruits.len();
        let top_left = Self::get_top_left(&fruits);
        let top_right = Self::get_top_right(&fruits);
        let bottom_left = Self::get_bottom_left(&fruits);
        top_left + top_right + bottom_left - 2 * fruits[n - 1][n - 1]
    }

    fn get_top_left(fruits: &Vec<Vec<i32>>) -> i32 {
        let n = fruits.len();
        let mut res = 0;
        for i in 0..n {
            res += fruits[i][i];
        }
        res
    }

    fn get_top_right(fruits: &Vec<Vec<i32>>) -> i32 {
        let n = fruits.len();
        let mut dp = vec![vec![0; n]; n];
        dp[0][n - 1] = fruits[0][n - 1];

        for x in 0..n {
            for y in 0..n {
                if x >= y && !(x == n - 1 && y == n - 1) {
                    continue;
                }

                for (dx, dy) in [(1, -1), (1, 0), (1, 1)] {
                    let i = x as isize - dx;
                    let j = y as isize - dy;

                    if i < 0 || i as usize >= n || j < 0 || j as usize >= n {
                        continue;
                    }

                    let i = i as usize;
                    let j = j as usize;

                    if i < j && j < n - 1 - i {
                        continue;
                    }

                    dp[x][y] = dp[x][y].max(dp[i][j] + fruits[x][y]);
                }
            }
        }

        dp[n - 1][n - 1]
    }

    fn get_bottom_left(fruits: &Vec<Vec<i32>>) -> i32 {
        let n = fruits.len();
        let mut dp = vec![vec![0; n]; n];
        dp[n - 1][0] = fruits[n - 1][0];

        for y in 0..n {
            for x in 0..n {
                if x <= y && !(x == n - 1 && y == n - 1) {
                    continue;
                }

                for (dx, dy) in [(-1, 1), (0, 1), (1, 1)] {
                    let i = x as isize - dx;
                    let j = y as isize - dy;

                    if i < 0 || i as usize >= n || j < 0 || j as usize >= n {
                        continue;
                    }

                    let i = i as usize;
                    let j = j as usize;

                    if j < i && i < n - 1 - j {
                        continue;
                    }

                    dp[x][y] = dp[x][y].max(dp[i][j] + fruits[x][y]);
                }
            }
        }

        dp[n - 1][n - 1]
    }
}
