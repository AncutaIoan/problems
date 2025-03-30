impl Solution {
    fn mark_island_dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize, m: usize, n:usize) {
        if grid[i][j] != '1' {
            return;
        }
        
        grid[i][j] = '#';
        
        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        for &(di, dj) in &directions {
            let new_i = i + di as usize;
            let new_j = j + dj as usize;

            if new_i >= 0 && new_j >= 0 && new_i < m  && new_j < n {
                Self::mark_island_dfs(grid, new_i, new_j, m, n);
            }
        }
    }

    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut number_of_islands = 0;
        let n = grid[0].len();
        let m = grid.len();
        for i in 0..m {
            for j in 0..n  {
                if grid[i][j] == '1' {
                    number_of_islands += 1;
                    Self::mark_island_dfs(&mut grid, i, j, m, n);
                }
            }
        }

        number_of_islands
    }
}
