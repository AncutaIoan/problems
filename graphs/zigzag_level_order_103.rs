/*
103. Binary Tree Zigzag Level Order Traversal

Given the root of a binary tree, return the zigzag level order traversal of its nodes' values. (i.e., from left to right, then right to left for the next level and alternate between).


*/


// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }

        let mut stack = vec![root.unwrap()]; // Start with the root node in the stack.
        let mut result = Vec::new();
        let mut switch = false; // A flag to alternate traversal directions.

        while !stack.is_empty() {
            let mut level = Vec::new();
            let mut next_level = Vec::new();

            // Traverse the current level nodes
            for node_rc in stack.drain(..) {
                let node = node_rc.borrow();
                level.push(node.val); // Add current node value to level.
                // Collect next level nodes in reverse order if switch is true
                if let Some(left) = &node.left {
                    next_level.push(left.clone());
                }
                if let Some(right) = &node.right {
                    next_level.push(right.clone());
                }

            }

            // If we are on an odd level, reverse the order of the `level`
            if switch {
                level.reverse();
            }

            // Add the current level to the result
            result.push(level);

            // Update stack for the next level, and flip the switch for zigzag direction
            stack = next_level;
            switch = !switch;
        }

        result
    }
}

