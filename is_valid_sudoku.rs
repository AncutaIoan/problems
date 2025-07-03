/*
36. Valid Sudoku

Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:

Each row must contain the digits 1-9 without repetition.
Each column must contain the digits 1-9 without repetition.
Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
Note:

A Sudoku board (partially filled) could be valid but is not necessarily solvable.
Only the filled cells need to be validated according to the mentioned rules.
*/
use std::collections::{HashSet};

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // Track digits seen in each row, column, and block (3x3 sub-box)
        let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut blocks: Vec<HashSet<char>> = vec![HashSet::new(); 9];

        for i in 0..9 {
            for j in 0..9 {
                let c = board[i][j];
                if c == '.' {
                    continue; // skip empty cells
                }
                // Check row
                if rows[i].contains(&c) {
                    return false;
                }
                rows[i].insert(c);

                // Check column
                if cols[j].contains(&c) {
                    return false;
                }
                cols[j].insert(c);

                // Calculate block index: each block is 3x3
                let block_index = (i / 3) * 3 + (j / 3);
                if blocks[block_index].contains(&c) {
                    return false;
                }
                blocks[block_index].insert(c);
            }
        }

        true
    }
}
