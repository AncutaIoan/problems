/*
498. Diagonal Traverse

Given an m x n matrix mat, return an array of all the elements of the array in a diagonal order.



Explanation : 
Explanation:
Initialize pointers: Start at the top-left corner of the matrix (row = 0, col = 0).

Traverse diagonally:

If moving upwards, go diagonally up-right (row - 1, col + 1).

If moving downwards, go diagonally down-left (row + 1, col - 1).

Handle boundaries:

If we reach the last column, move down and switch direction.

If we reach the first row, move right and switch direction.

If we reach the last row, move right and switch direction.

If we reach the first column, move down and switch direction.

Repeat until all elements are visited.



*/


impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        if mat.is_empty() || mat[0].is_empty() {
            return vec![];
        }
        
        let m = mat.len();
        let n = mat[0].len();
        let mut result = Vec::with_capacity(m * n);
        let mut row = 0;
        let mut col = 0;
        let mut up = true;
        
        for _ in 0..(m * n) {
            result.push(mat[row][col]);
            if up {
                if col == n - 1 {
                    row += 1;
                    up = false;
                } else if row == 0 {
                    col += 1;
                    up = false;
                } else {
                    row -= 1;
                    col += 1;
                }
            } else {
                if row == m - 1 {
                    col += 1;
                    up = true;
                } else if col == 0 {
                    row += 1;
                    up = true;
                } else {
                    row += 1;
                    col -= 1;
                }
            }
        }
        
        result
    }
}
