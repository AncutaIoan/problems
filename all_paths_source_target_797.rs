
//V1 DFS (ideomatic, could be made better (getting rid of start/finish params etc)
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(
            target: usize,
            current_node: usize,
            graph: &Vec<Vec<i32>>,
            current_iteration: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if current_node == target {
                current_iteration.push(current_node as i32);
                result.push(current_iteration.clone());
                current_iteration.pop(); 
                return;
            }

            current_iteration.push(current_node as i32); 

            for &next_node in &graph[current_node] {
                dfs(target, next_node as usize, graph, current_iteration, result);
            }

            current_iteration.pop(); 
        }

        let mut result = Vec::new();
        let target = graph.len() - 1; 

        let mut current_iteration = Vec::new();
        dfs(target, 0, &graph, &mut current_iteration, &mut result);

        result
    }
}

//V2 stack approach

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let target = graph.len() - 1;

        let mut stack = vec![(0, vec![])];
        
        while let Some((current_node, mut path)) = stack.pop() {
            path.push(current_node as i32);

            // If we reach the target node, store the path
            if current_node == target {
                result.push(path);
            } else {
                // Push the adjacent nodes onto the stack
                for &next_node in &graph[current_node] {
                    stack.push((next_node as usize, path.clone()));
                }
            }
        }

        result
    }
}
