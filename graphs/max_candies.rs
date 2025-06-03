/*
1298. Maximum Candies You Can Get from Boxes

You have n boxes labeled from 0 to n - 1. You are given four arrays: status, candies, keys, and containedBoxes where:

status[i] is 1 if the ith box is open and 0 if the ith box is closed,
candies[i] is the number of candies in the ith box,
keys[i] is a list of the labels of the boxes you can open after opening the ith box.
containedBoxes[i] is a list of the boxes you found inside the ith box.
You are given an integer array initialBoxes that contains the labels of the boxes you initially have. You can take all the candies in any open box and you can use the keys in it to open new boxes and you also can use the boxes you find in it.

Return the maximum number of candies you can get following the rules above.


*/

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>
    ) -> i32 {
        let n = status.len();
        let mut has_key = vec![false; n];
        let mut has_box = vec![false; n];
        let mut visited = vec![false; n];

        // Initialize with initial boxes
        for &box_id in &initial_boxes {
            has_box[box_id as usize] = true;
        }

        let mut total_candies = 0;
        let mut found_new = true;

        // Keep processing as long as we're unlocking new opportunities
        while found_new {
            found_new = false;
            for i in 0..n {
                if !visited[i] && has_box[i] && (status[i] == 1 || has_key[i]) {
                    visited[i] = true;
                    found_new = true;
                    total_candies += candies[i];

                    // Collect keys
                    for &k in &keys[i] {
                        has_key[k as usize] = true;
                    }

                    // Collect new boxes
                    for &b in &contained_boxes[i] {
                        has_box[b as usize] = true;
                    }
                }
            }
        }

        total_candies
    }
}
