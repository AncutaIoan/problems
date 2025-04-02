/*
120. Triangle
Given a triangle array, return the minimum path sum from top to bottom.

For each step, you may move to an adjacent number of the row below. More formally, if you are on index i on the current row, you may move to either index i or index i + 1 on the next row.

 

Example 1:

Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
Output: 11
Explanation: The triangle looks like:
   2
  3 4
 6 5 7
4 1 8 3
The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11 (underlined above).



*/

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut triangle = triangle;

        // Iterate from the second-last row to the top
        for row in (0..triangle.len() - 1).rev() {
            for col in 0..triangle[row].len() {
                // For each element, we choose the minimum path sum from the two elements below
                triangle[row][col] += triangle[row + 1][col].min(triangle[row + 1][col + 1]);
            }
        }

        // The top element now contains the minimum path sum
        triangle[0][0]
    }
}
