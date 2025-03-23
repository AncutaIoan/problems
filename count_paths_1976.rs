impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut graph = vec![vec![]; n as usize];
        let mut min_dist_vec = vec![i64::MAX; n as usize];

        // Build the graph
        for road in roads {
            let (i, v, time) = (road[0] as usize, road[1] as usize, road[2] as i64);
            graph[i].push((v, time));
            graph[v].push((i, time));
        }

        // Dijkstra to find the shortest distances
        let mut heap = std::collections::BinaryHeap::new();
        min_dist_vec[0] = 0;
        heap.push((0, 0)); // (distance, node)

        while let Some((dist, u)) = heap.pop() {
            let dist = -dist; // Rust's BinaryHeap is a max-heap, so use negative values for min-heap

            if dist > min_dist_vec[u] {
                continue;
            }

            for &(v, time) in &graph[u] {
                let new_dist = dist + time;
                if new_dist < min_dist_vec[v] {
                    min_dist_vec[v] = new_dist;
                    heap.push((-new_dist, v));
                }
            }
        }

        // DFS + Memoization to count paths
        fn dfs(node: usize, n: usize, graph: &Vec<Vec<(usize, i64)>>, min_dist_vec: &Vec<i64>, memo: &mut Vec<i64>) -> i64 {
            if node == n - 1 {
                return 1; // Reached destination
            }
            if memo[node] != -1 {
                return memo[node]; // Return cached result
            }

            let mut count = 0;
            for &(next, time) in &graph[node] {
                if min_dist_vec[next] == min_dist_vec[node] + time {
                    count = (count + dfs(next, n, graph, min_dist_vec, memo)) % MOD;
                }
            }

            memo[node] = count;
            count
        }

        let mut memo = vec![-1; n as usize];
        dfs(0, n as usize, &graph, &min_dist_vec, &mut memo) as i32
    }
}
