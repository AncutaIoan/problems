/*

3373. Maximize the Number of Target Nodes After Connecting Trees II

There exist two undirected trees with n and m nodes, labeled from [0, n - 1] and [0, m - 1], respectively.

You are given two 2D integer arrays edges1 and edges2 of lengths n - 1 and m - 1, respectively, where edges1[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the first tree and edges2[i] = [ui, vi] indicates that there is an edge between nodes ui and vi in the second tree.

Node u is target to node v if the number of edges on the path from u to v is even. Note that a node is always target to itself.

Return an array of n integers answer, where answer[i] is the maximum possible number of nodes that are target to node i of the first tree if you had to connect one node from the first tree to another node in the second tree.

Note that queries are independent from each other. That is, for every query you will remove the added edge before proceeding to the next query.

 

Example 1:

Input: edges1 = [[0,1],[0,2],[2,3],[2,4]], edges2 = [[0,1],[0,2],[0,3],[2,7],[1,4],[4,5],[4,6]]

Output: [8,7,7,8,8]

Explanation:

For i = 0, connect node 0 from the first tree to node 0 from the second tree.
For i = 1, connect node 1 from the first tree to node 4 from the second tree.
For i = 2, connect node 2 from the first tree to node 7 from the second tree.
For i = 3, connect node 3 from the first tree to node 0 from the second tree.
For i = 4, connect node 4 from the first tree to node 4 from the second tree.

*/
use std::collections::VecDeque;

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        // Helper BFS function:
        // - Returns number of nodes at even levels
        // - Optionally marks those nodes in `included`
        fn bfs(
            start: usize,
            adj: &Vec<Vec<usize>>,
            mut included: Option<&mut Vec<bool>>,
        ) -> i32 {
            let mut queue = VecDeque::new();
            queue.push_back((start, usize::MAX));
            let mut level = 0;
            let mut count = 0;

            while !queue.is_empty() {
                let size = queue.len();

                if level % 2 == 0 {
                    count += size as i32;
                }

                for _ in 0..size {
                    let (curr, parent) = queue.pop_front().unwrap();

                    if let Some(included_vec) = included.as_mut() {
                        if level % 2 == 0 {
                            included_vec[curr] = true;
                        }
                    }

                    for &neighbor in &adj[curr] {
                        if neighbor != parent {
                            queue.push_back((neighbor, curr));
                        }
                    }
                }

                level += 1;
            }

            count
        }

        // Build adjacency list from edge list
        fn build_adj(n: usize, edges: &[Vec<i32>]) -> Vec<Vec<usize>> {
            let mut adj = vec![vec![]; n];
            for edge in edges {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                adj[u].push(v);
                adj[v].push(u);
            }
            adj
        }

        let n1 = edges1.len() + 1;
        let n2 = edges2.len() + 1;

        let adj1 = build_adj(n1, &edges1);
        let adj2 = build_adj(n2, &edges2);

        // Step 1: Count even-level nodes in tree2
        let even_count2 = bfs(0, &adj2, None);
        let odd_count2 = n2 as i32 - even_count2;
        let best2 = even_count2.max(odd_count2);

        // Step 2: Mark even-level nodes in tree1
        let mut included = vec![false; n1];
        let even_count1 = bfs(0, &adj1, Some(&mut included));

        // Step 3: For each node in tree1, compute max target nodes
        let mut ans = vec![0; n1];
        for i in 0..n1 {
            if included[i] {
                ans[i] = even_count1 + best2;
            } else {
                ans[i] = (n1 as i32 - even_count1) + best2;
            }
        }

        ans
    }
}
