use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        // Build adjacency list
        let mut graph = vec![Vec::<(usize, i32)>::new(); n];

        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let w = e[2];

            // original direction
            graph[u].push((v, w));
            // reversed direction with doubled cost
            graph[v].push((u, w * 2));
        }

        // Dijkstra
        let mut dist = vec![i32::MAX; n];
        dist[0] = 0;

        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), 0usize));

        while let Some((Reverse(cost), u)) = heap.pop() {
            if cost > dist[u] {
                continue;
            }

            if u == n - 1 {
                return cost;
            }

            for &(v, w) in &graph[u] {
                let next = cost + w;
                if next < dist[v] {
                    dist[v] = next;
                    heap.push((Reverse(next), v));
                }
            }
        }

        dist[n - 1]
    }
}
