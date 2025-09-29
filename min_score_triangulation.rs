/*
1039. Minimum Score Triangulation of Polygon

You have a convex n-sided polygon where each vertex has an integer value. You are given an integer array values where values[i] is the value of the ith vertex in clockwise order.

Polygon triangulation is a process where you divide a polygon into a set of triangles and the vertices of each triangle must also be vertices of the original polygon. Note that no other shapes other than triangles are allowed in the division. This process will result in n - 2 triangles.

You will triangulate the polygon. For each triangle, the weight of that triangle is the product of the values at its vertices. The total score of the triangulation is the sum of these weights over all n - 2 triangles.

Return the minimum possible score that you can achieve with some triangulation of the polygon.
*/
impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let n = values.len();
        // dp[i][j] = minimum triangulation score for sub-polygon vertices i..=j
        let mut dp = vec![vec![0; n]; n];

        // We fill for intervals of length >= 3
        for len in 2..n {                  // len is j - i
            for i in 0..n - len {
                let j = i + len;
                dp[i][j] = i32::MAX;
                for k in i + 1..j {
                    let cost = dp[i][k] + dp[k][j] + values[i] * values[j] * values[k];
                    dp[i][j] = dp[i][j].min(cost);
                }
            }
        }

        dp[0][n - 1]
    }
}
