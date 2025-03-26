impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let m = matrix[0].len();

        let mut first_row_has_zero = false;
        let mut first_col_has_zero = false;

        // Check if the first row has zero
        for j in 0..m {
            if matrix[0][j] == 0 {
                first_row_has_zero = true;
                break;
            }
        }

        // Check if the first column has zero
        for i in 0..n {
            if matrix[i][0] == 0 {
                first_col_has_zero = true;
                break;
            }
        }

        // Mark zeroes using first row and column
        for i in 1..n {
            for j in 1..m {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        // Zero out marked rows
        for i in 1..n {
            if matrix[i][0] == 0 {
                matrix[i] = vec![0; m];
            }
        }

        // Zero out marked columns
        for j in 1..m {
            if matrix[0][j] == 0 {
                for i in 0..n {
                    matrix[i][j] = 0;
                }
            }
        }

        // Zero out first row if needed
        if first_row_has_zero {
            matrix[0] = vec![0; m];
        }

        // Zero out first column if needed
        if first_col_has_zero {
            for i in 0..n {
                matrix[i][0] = 0;
            }
        }
    }
}
