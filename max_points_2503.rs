use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut result = vec![0; queries.len()];
        
        let mut queries_with_index: Vec<(i32, usize)> = queries.iter().enumerate().map(|(i, &q)| (q, i)).collect();
        queries_with_index.sort_unstable(); // Sort queries in ascending order
        
        let mut visited = vec![vec![false; cols]; rows];
        let mut heap = BinaryHeap::new(); // Min-heap using Reverse
        
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        
        heap.push((Reverse(grid[0][0]), 0, 0)); // Start from the top-left corner
        visited[0][0] = true;
        
        let mut count = 0;
        let mut threshold = 0;
        let mut query_index = 0;
        
        while query_index < queries_with_index.len() {
            let (current_query, original_index) = queries_with_index[query_index];

            while let Some((Reverse(value), r, c)) = heap.pop() {
                if value >= current_query { // Stop when we reach a higher value
                    heap.push((Reverse(value), r, c));
                    break;
                }

                count += 1; // Count this cell as part of the answer

                for (dr, dc) in &directions {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;

                    if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                        let (nr, nc) = (nr as usize, nc as usize);
                        if !visited[nr][nc] {
                            visited[nr][nc] = true;
                            heap.push((Reverse(grid[nr][nc]), nr, nc));
                        }
                    }
                }
            }
            
            result[original_index] = count; // Store result for this query
            query_index += 1;
        }
        
        result
    }
}



/*
Key points, this can be updated to make use of precomputed values to save some time, but still fail for big datasets
impl Solution {

    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        fn add_border(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let rows = matrix.len();
            let cols = if rows > 0 { matrix[0].len() } else { 0 };

            let mut new_matrix = vec![vec![0; cols + 2]; rows + 2];

            for i in 0..rows {
                for j in 0..cols {
                    new_matrix[i + 1][j + 1] = matrix[i][j];
                }
            }

            new_matrix
        }

        fn dfs(grid: &mut Vec<Vec<i32>>, current_i: usize, current_j: usize, current_query: i32) -> i32 {
            if current_i == 0 || current_i == grid.len() - 1 || current_j == 0 || current_j == grid[0].len() - 1 {
                return 0;
            }
            if grid[current_i][current_j] == 0 || current_query <= grid[current_i][current_j] {
                return 0;
            }

            let mut count = 1; // Include the current cell
            grid[current_i][current_j] = 0; // Mark as visited
            
            let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
            for (di, dj) in dirs {
                let ni = current_i as isize + di;
                let nj = current_j as isize + dj;

                if ni > 0 && ni < grid.len() as isize - 1 && nj > 0 && nj < grid[0].len() as isize - 1 {
                    count += dfs(grid, ni as usize, nj as usize, current_query);
                }
            }

            count
        }

        let grid_bord = add_border(&grid); // Keep original grid_bord unchanged
        let mut result = vec![0; queries.len()];

        for (i, &q) in queries.iter().enumerate() {
            let mut fresh_grid = grid_bord.clone(); // Clone for each query
            result[i] = dfs(&mut fresh_grid, 1, 1, q); // Pass mutable reference
        }

        result
    }
    
}
*/
