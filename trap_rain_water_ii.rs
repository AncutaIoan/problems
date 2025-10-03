/*
407. Trapping Rain Water II

Given an m x n integer matrix heightMap representing the height of each unit cell in a 2D elevation map, return the volume of water it can trap after raining
*/
use std::collections::BinaryHeap;
use std::cmp::{Reverse, max};

impl Solution {
    pub fn trap_rain_water(mut height_map: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (height_map.len(), height_map[0].len());
        if rows < 3 || cols < 3 {
            return 0; // No enclosed water possible
        }

        let mut min_heap = BinaryHeap::new();
        let mut visited = vec![vec![false; cols]; rows];

        // Add boundary cells to min-heap and mark them as visited
        for i in 0..rows {
            for j in 0..cols {
                if i == 0 || i == rows - 1 || j == 0 || j == cols - 1 {
                    min_heap.push(Reverse((height_map[i][j], i, j)));
                    visited[i][j] = true;
                }
            }
        }

        let mut res = 0;
        let mut max_height = 0;
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        while let Some(Reverse((h, r, c))) = min_heap.pop() {
            max_height = max(max_height, h);

            for (dr, dc) in &directions {
                let (nr, nc) = (r as isize + dr, c as isize + dc);

                if nr >= 0 && nc >= 0 && (nr as usize) < rows && (nc as usize) < cols {
                    let (nr, nc) = (nr as usize, nc as usize);
                    if !visited[nr][nc] {
                        visited[nr][nc] = true;
                        res += max(0, max_height - height_map[nr][nc]);
                        min_heap.push(Reverse((height_map[nr][nc], nr, nc)));
                    }
                }
            }
        }

        res
    }
}
