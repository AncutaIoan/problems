impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let (graph, in_degree) = Self::build_graph_and_in_degree(num_courses, prerequisites);
        let queue = Self::initialize_queue(num_courses, &in_degree);
        Self::process_courses(num_courses, graph, in_degree, queue)
    }

    // Builds the graph and in-degree array from the prerequisites
    fn build_graph_and_in_degree(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> (std::collections::HashMap<i32, Vec<i32>>, Vec<i32>) {
        let mut graph: std::collections::HashMap<i32, Vec<i32>> = std::collections::HashMap::new();
        let mut in_degree: Vec<i32> = vec![0; num_courses as usize];

        for prereq in prerequisites {
            let course = prereq[0];
            let prerequisite = prereq[1];
            graph.entry(prerequisite).or_insert(vec![]).push(course);
            in_degree[course as usize] += 1;
        }

        (graph, in_degree)
    }

    // Initializes the queue with all courses that have no prerequisites (in-degree = 0)
    fn initialize_queue(num_courses: i32, in_degree: &Vec<i32>) -> std::collections::VecDeque<i32> {
        let mut queue: std::collections::VecDeque<i32> = std::collections::VecDeque::new();

        for i in 0..num_courses {
            if in_degree[i as usize] == 0 {
                queue.push_back(i);
            }
        }

        queue
    }

    // Processes courses by performing a topological sort and checking for cycles
    fn process_courses(
        num_courses: i32,
        graph: std::collections::HashMap<i32, Vec<i32>>,
        mut in_degree: Vec<i32>,
        mut queue: std::collections::VecDeque<i32>,
    ) -> bool {
        let mut processed_courses = 0;

        while let Some(course) = queue.pop_front() {
            processed_courses += 1;

            if let Some(neighbors) = graph.get(&course) {
                for &neighbor in neighbors {
                    in_degree[neighbor as usize] -= 1;
                    if in_degree[neighbor as usize] == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        processed_courses == num_courses
    }
}
