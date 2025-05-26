/*

1857. Largest Color Value in a Directed Graph
There is a directed graph of n colored nodes and m edges. The nodes are numbered from 0 to n - 1.

You are given a string colors where colors[i] is a lowercase English letter representing the color of the ith node in this graph (0-indexed). You are also given a 2D array edges where edges[j] = [aj, bj] indicates that there is a directed edge from node aj to node bj.

A valid path in the graph is a sequence of nodes x1 -> x2 -> x3 -> ... -> xk such that there is a directed edge from xi to xi+1 for every 1 <= i < k. The color value of the path is the number of nodes that are colored the most frequently occurring color along that path.

Return the largest color value of any valid path in the given graph, or -1 if the graph contains a cycle.

*/
use std::collections::{VecDeque, HashMap};

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let n = colors.len();
        let mut graph = vec![vec![]; n];
        let mut indegree = vec![0; n];

        // Build graph
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            indegree[v] += 1;
        }

        let mut queue = VecDeque::new();
        let mut color_count = vec![vec![0; 26]; n];
        let colors_bytes = colors.as_bytes();

        // Start with all nodes having indegree 0
        for i in 0..n {
            if indegree[i] == 0 {
                queue.push_back(i);
            }
        }

        let mut visited = 0;
        let mut max_color = 0;

        while let Some(node) = queue.pop_front() {
            visited += 1;
            let color_idx = (colors_bytes[node] - b'a') as usize;
            color_count[node][color_idx] += 1;
            max_color = max_color.max(color_count[node][color_idx]);

            for &neighbor in &graph[node] {
                for c in 0..26 {
                    color_count[neighbor][c] = color_count[neighbor][c].max(color_count[node][c]);
                }
                indegree[neighbor] -= 1;
                if indegree[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }

        if visited != n {
            return -1; // Cycle detected
        }

        max_color
    }
}
