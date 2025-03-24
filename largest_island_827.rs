impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        use std::collections::HashSet;

        fn border_with_zeros(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let rows = matrix.len();
            let cols = if rows > 0 { matrix[0].len() } else { 0 };
            let mut bordered = vec![vec![0; cols + 2]; rows + 2];

            for i in 0..rows {
                for j in 0..cols {
                    bordered[i + 1][j + 1] = matrix[i][j];
                }
            }

            bordered
        }

        fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize, island_id: i32) -> i32 {
            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            let mut stack = vec![(i, j)];
            let mut size = 0;

            while let Some((x, y)) = stack.pop() {
                if grid[x][y] != 1 {
                    continue;
                }
                grid[x][y] = island_id;
                size += 1;

                for (dx, dy) in directions.iter() {
                    let nx = (x as isize + dx) as usize;
                    let ny = (y as isize + dy) as usize;
                    if grid[nx][ny] == 1 {
                        stack.push((nx, ny));
                    }
                }
            }

            size
        }

        let mut bordered = border_with_zeros(&grid);
        let mut island_sizes = HashMap::new();
        let mut current_island_id = 2;

        for i in 1..bordered.len() - 1 {
            for j in 1..bordered[0].len() - 1 {
                if bordered[i][j] == 1 {
                    let size = dfs(&mut bordered, i, j, current_island_id);
                    island_sizes.insert(current_island_id, size);
                    current_island_id += 1;
                }
            }
        }

        let mut max_size = *island_sizes.values().max().unwrap_or(&0);

        // Try converting a zero to a bridge and calculate new island size
        for i in 1..bordered.len() - 1 {
            for j in 1..bordered[0].len() - 1 {
                if bordered[i][j] == 0 {
                    let mut seen = HashSet::new();
                    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

                    for (dx, dy) in directions.iter() {
                        let nx = (i as isize + dx) as usize;
                        let ny = (j as isize + dy) as usize;
                        if bordered[nx][ny] > 1 {
                            seen.insert(bordered[nx][ny]);
                        }
                    }

                    let new_size = 1 + seen.iter().map(|&id| island_sizes.get(&id).unwrap_or(&0)).sum::<i32>();
                    max_size = max_size.max(new_size);
                }
            }
        }

        max_size
    }
}
