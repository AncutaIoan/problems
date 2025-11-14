/*
2536. Increment Submatrices by One

You are given a positive integer n, indicating that we initially have an n x n 0-indexed integer matrix mat filled with zeroes.

You are also given a 2D integer array query. For each query[i] = [row1i, col1i, row2i, col2i], you should do the following operation:

Add 1 to every element in the submatrix with the top left corner (row1i, col1i) and the bottom right corner (row2i, col2i). That is, add 1 to mat[x][y] for all row1i <= x <= row2i and col1i <= y <= col2i.
Return the matrix mat after performing every query.
*/
impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut diff = vec![vec![0; n + 1]; n + 1];

        // Mark difference array updates
        for q in queries {
            let r1 = q[0] as usize;
            let c1 = q[1] as usize;
            let r2 = q[2] as usize;
            let c2 = q[3] as usize;

            diff[r1][c1] += 1;
            diff[r1][c2 + 1] -= 1;
            diff[r2 + 1][c1] -= 1;
            diff[r2 + 1][c2 + 1] += 1;
        }

        // Prefix sum horizontally
        for r in 0..n {
            for c in 1..n {
                diff[r][c] += diff[r][c - 1];
            }
        }

        // Prefix sum vertically
        for r in 1..n {
            for c in 0..n {
                diff[r][c] += diff[r - 1][c];
            }
        }

        // Build result trimming extra row/col
        let mut result = vec![vec![0; n]; n];
        for r in 0..n {
            for c in 0..n {
                result[r][c] = diff[r][c];
            }
        }

        result
    }
}
