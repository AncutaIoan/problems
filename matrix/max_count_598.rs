/*
598. Range Addition II

You are given an m x n matrix M initialized with all 0's and an array of operations ops, where ops[i] = [ai, bi] means M[x][y] should be incremented by one for all 0 <= x < ai and 0 <= y < bi.

Count and return the number of maximum integers in the matrix after performing all the operations.



*/


impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        if ops.is_empty() {
            return m * n; 
        }

        let min_ai = ops.iter().map(|op| op[0]).min().unwrap();
        let min_bi = ops.iter().map(|op| op[1]).min().unwrap();

        min_ai * min_bi
    }
}
