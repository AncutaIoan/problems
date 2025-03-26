struct FenwickTree {
    tree: Vec<i32>,
}

impl FenwickTree {
    fn new(size: usize) -> Self {
        FenwickTree { tree: vec![0; size + 1] }
    }

    fn update(&mut self, mut index: usize, value: i32) {
        while index < self.tree.len() {
            self.tree[index] += value;
            index += index & (!index + 1);
        }
    }

    fn query(&self, mut index: usize) -> i32 {
        let mut sum = 0;
        while index > 0 {
            sum += self.tree[index];
            index -= index & (!index + 1);
        }
        sum
    }
}


impl Solution {
    /*
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
  
        Bad solution for performance
        nums.iter()
            .enumerate() // Track index and value
            .map(|(i, &x)| {
                nums.iter()
                    .skip(i)
                    .enumerate()
                    .filter(|(j, &y)| y < x) // y must be after x and smaller
                    .count() as i32
            })
            .collect()
    }
    */
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];
        if n == 0 {
            return result;
        }

        // Coordinate compression: Map values to a smaller range
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_unstable();
        let get_index = |x: i32| sorted_nums.binary_search(&x).unwrap() + 1;

        // Fenwick Tree to store frequency of elements
        let mut fenwick = FenwickTree::new(n);

        // Traverse from right to left
        for (i, &num) in nums.iter().enumerate().rev() {
            let idx = get_index(num);
            // Count how many numbers are smaller than the current one
            result[i] = fenwick.query(idx - 1);
            // Update the Fenwick Tree with the current number
            fenwick.update(idx, 1);
        }

        result
    }
}
