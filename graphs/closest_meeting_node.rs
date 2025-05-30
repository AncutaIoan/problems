/*

2359. Find Closest Node to Given Two Nodes

You are given a directed graph of n nodes numbered from 0 to n - 1, where each node has at most one outgoing edge.

The graph is represented with a given 0-indexed array edges of size n, indicating that there is a directed edge from node i to node edges[i]. If there is no outgoing edge from i, then edges[i] == -1.

You are also given two integers node1 and node2.

Return the index of the node that can be reached from both node1 and node2, such that the maximum between the distance from node1 to that node, and from node2 to that node is minimized. If there are multiple answers, return the node with the smallest index, and if no possible answer exists, return -1.

Note that edges may contain cycles.

*/

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        fn get_distances(start: i32, edges: &Vec<i32>) -> Vec<i32> {
            let n = edges.len();
            let mut dist = vec![-1; n];
            let mut current = start;
            let mut d = 0;
            while current != -1 && dist[current as usize] == -1 {
                dist[current as usize] = d;
                d += 1;
                current = edges[current as usize];
            }
            dist
        }

        let dist1 = get_distances(node1, &edges);
        let dist2 = get_distances(node2, &edges);
        let mut result = -1;
        let mut min_dist = std::i32::MAX;

        for i in 0..edges.len() {
            if dist1[i] != -1 && dist2[i] != -1 {
                let max_dist = dist1[i].max(dist2[i]);
                if max_dist < min_dist {
                    min_dist = max_dist;
                    result = i as i32;
                }
            }
        }

        result
    }
}
