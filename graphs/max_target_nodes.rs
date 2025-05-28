/*
3372. Maximize the Number of Target Nodes After Connecting Trees I
There exist two undirected trees with n and m nodes, with distinct labels in ranges [0, n - 1] and [0, m - 1], respectively.

You are given two 2D integer arrays edges1 and edges2 of lengths n - 1 and m - 1, respectively, where edges1[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the first tree and edges2[i] = [ui, vi] indicates that there is an edge between nodes ui and vi in the second tree. You are also given an integer k.

Node u is target to node v if the number of edges on the path from u to v is less than or equal to k. Note that a node is always target to itself.

Return an array of n integers answer, where answer[i] is the maximum possible number of nodes target to node i of the first tree if you have to connect one node from the first tree to another node in the second tree.

Note that queries are independent from each other. That is, for every query you will remove the added edge before proceeding to the next query.

 

Example 1:

Input: edges1 = [[0,1],[0,2],[2,3],[2,4]], edges2 = [[0,1],[0,2],[0,3],[2,7],[1,4],[4,5],[4,6]], k = 2

Output: [9,7,9,8,8]

Explanation:

For i = 0, connect node 0 from the first tree to node 0 from the second tree.
For i = 1, connect node 1 from the first tree to node 0 from the second tree.
For i = 2, connect node 2 from the first tree to node 4 from the second tree.
For i = 3, connect node 3 from the first tree to node 4 from the second tree.
For i = 4, connect node 4 from the first tree to node 4 from the second tree.


*/
use std::collections::VecDeque;

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;
        let k = k as usize;

        // Helper: Build adjacency list
        fn build_tree(edges: &Vec<Vec<i32>>, size: usize) -> Vec<Vec<usize>> {
            let mut g = vec![vec![]; size];
            for edge in edges {
                let (a, b) = (edge[0] as usize, edge[1] as usize);
                g[a].push(b);
                g[b].push(a);
            }
            g
        }

        // Helper: BFS to count how many nodes are within distance `max_depth` from `start`
        fn bfs_count(graph: &Vec<Vec<usize>>, start: usize, max_depth: usize) -> usize {
            let mut visited = vec![false; graph.len()];
            let mut queue = VecDeque::new();
            visited[start] = true;
            queue.push_back((start, 0));
            let mut count = 0;

            while let Some((node, dist)) = queue.pop_front() {
                if dist > max_depth {
                    continue;
                }
                count += 1;
                for &nei in &graph[node] {
                    if !visited[nei] {
                        visited[nei] = true;
                        queue.push_back((nei, dist + 1));
                    }
                }
            }
            count
        }

        let g1 = build_tree(&edges1, n);
        let g2 = build_tree(&edges2, m);

        // Step 1: Count reachable nodes from each node in Tree 1 within distance k
        let mut cnt1 = vec![0; n];
        for i in 0..n {
            cnt1[i] = bfs_count(&g1, i, k);
        }

        // Step 2: For Tree 2, compute max reachable nodes within distance d (for d in 0..k)
        let mut cnt2 = vec![0; k];
        for i in 0..m {
            for d in 0..k {
                let c = bfs_count(&g2, i, d);
                cnt2[d] = cnt2[d].max(c);
            }
        }

        // Step 3: Combine results
        let mut answer = vec![0; n];
        for i in 0..n {
            let extra = if k >= 1 { cnt2[k - 1] } else { 0 };
            answer[i] = (cnt1[i] + extra) as i32;
        }

        answer
    }
}
