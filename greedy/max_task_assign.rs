use std::collections::VecDeque;

impl Solution {
    pub fn max_task_assign(mut tasks: Vec<i32>, mut workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
        tasks.sort();
        workers.sort();

        let num_tasks = tasks.len();
        let num_workers = workers.len();

        fn can_assign(x: usize, tasks: &Vec<i32>, workers: &Vec<i32>, pills: i32, strength: i32) -> bool {
            let mut task_index = 0;
            let mut queue: VecDeque<i32> = VecDeque::new();
            let mut remaining_pills = pills;

            for i in (workers.len() - x)..workers.len() {
                while task_index < x && tasks[task_index] <= workers[i] + strength {
                    queue.push_back(tasks[task_index]);
                    task_index += 1;
                }

                if queue.is_empty() {
                    return false;
                }

                if queue.front().unwrap() <= &workers[i] {
                    queue.pop_front();
                } else if remaining_pills > 0 {
                    queue.pop_back();
                    remaining_pills -= 1;
                } else {
                    return false;
                }
            }
            true
        }

        let mut left = 0;
        let mut right = num_tasks.min(num_workers);

        while left < right {
            let mid = (left + right + 1) / 2;
            if can_assign(mid, &tasks, &workers, pills, strength) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        left as i32
    }
}
