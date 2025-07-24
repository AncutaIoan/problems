/*
2322. Minimum Score After Removals on a Tree

There is an undirected connected tree with n nodes labeled from 0 to n - 1 and n - 1 edges.

You are given a 0-indexed integer array nums of length n where nums[i] represents the value of the ith node. You are also given a 2D integer array edges of length n - 1 where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree.

Remove two distinct edges of the tree to form three connected components. For a pair of removed edges, the following steps are defined:

Get the XOR of all the values of the nodes for each of the three components respectively.
The difference between the largest XOR value and the smallest XOR value is the score of the pair.
For example, say the three components have the node values: [4,5,7], [1,9], and [3,3,3]. The three XOR values are 4 ^ 5 ^ 7 = 6, 1 ^ 9 = 8, and 3 ^ 3 ^ 3 = 3. The largest XOR value is 8 and the smallest XOR value is 3. The score is then 8 - 3 = 5.
Return the minimum score of any possible pair of edge removals on the given tree.
*/

use std::collections::VecDeque;

impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut graph = vec![vec![]; n];

        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut in_time = vec![0; n];
        let mut out_time = vec![0; n];
        let mut xor = vec![0; n];
        let mut time = 0;

        // DFS to compute subtree XORs and in/out times
        fn dfs(
            node: usize,
            parent: usize,
            graph: &Vec<Vec<usize>>,
            nums: &Vec<i32>,
            in_time: &mut Vec<usize>,
            out_time: &mut Vec<usize>,
            xor: &mut Vec<i32>,
            time: &mut usize,
        ) {
            *time += 1;
            in_time[node] = *time;
            xor[node] = nums[node];
            for &child in &graph[node] {
                if child != parent {
                    dfs(child, node, graph, nums, in_time, out_time, xor, time);
                    xor[node] ^= xor[child];
                }
            }
            *time += 1;
            out_time[node] = *time;
        }

        dfs(0, usize::MAX, &graph, &nums, &mut in_time, &mut out_time, &mut xor, &mut time);
        let total_xor = xor[0];
        let mut min_score = i32::MAX;

        // Try all pairs of nodes (each node represents a subtree by cutting its parent edge)
        for u in 1..n {
            for v in 1..n {
                if u == v {
                    continue;
                }

                let (a, b, c) = if is_ancestor(u, v, &in_time, &out_time) {
                    // v is inside u
                    let x = xor[v];
                    let y = xor[u] ^ x;
                    let z = total_xor ^ xor[u];
                    (x, y, z)
                } else if is_ancestor(v, u, &in_time, &out_time) {
                    // u is inside v
                    let x = xor[u];
                    let y = xor[v] ^ x;
                    let z = total_xor ^ xor[v];
                    (x, y, z)
                } else {
                    // Disjoint
                    let x = xor[u];
                    let y = xor[v];
                    let z = total_xor ^ x ^ y;
                    (x, y, z)
                };

                let max_val = a.max(b).max(c);
                let min_val = a.min(b).min(c);
                min_score = min_score.min(max_val - min_val);
            }
        }

        min_score
    }
}

// Utility to check if u is ancestor of v using in/out times
fn is_ancestor(u: usize, v: usize, in_time: &Vec<usize>, out_time: &Vec<usize>) -> bool {
    in_time[u] < in_time[v] && out_time[u] > out_time[v]
}
