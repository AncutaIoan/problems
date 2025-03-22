impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); n];
        for edge in edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            adj_list[a].push(b);
            adj_list[b].push(a);
        }

        fn dfs(node: usize, adj_list: &Vec<Vec<usize>>, visited: &mut Vec<bool>, component: &mut Vec<usize>) {
            visited[node] = true;
            component.push(node);
            for &neighbor in adj_list[node].iter() {
                if !visited[neighbor] {
                    dfs(neighbor, adj_list, visited, component);
                }
            }
        }

        fn is_complete(component: &Vec<usize>, adj_list: &Vec<Vec<usize>>) -> bool {
            let size = component.len();
            let mut edge_count = 0;
            for &node in component.iter() {
                edge_count += adj_list[node].len();
            }
            edge_count / 2 == size * (size - 1) / 2
        }

        let mut visited = vec![false; n];
        let mut complete_count = 0;

        for i in 0..n {
            if !visited[i] {
                let mut component = Vec::new();
                dfs(i, &adj_list, &mut visited, &mut component);
                if is_complete(&component, &adj_list) {
                    complete_count += 1;
                }
            }
        }

        complete_count as i32
    }
}
