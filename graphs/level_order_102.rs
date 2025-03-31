/*
102. Binary Tree Level Order Traversal
Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).




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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }

        let mut stack = Vec::new();
        let mut result = Vec::new();

        stack.push(root.unwrap());

        while !stack.is_empty() {
            let mut level = Vec::new();
            let mut next_level = Vec::new();


            for node_rc in stack.drain(..) {
                let node = node_rc.borrow();
                level.push(node.val);


                if let Some(left) = &node.left {
                    next_level.push(left.clone());
                }

                
                if let Some(right) = &node.right {
                    next_level.push(right.clone());
                }
            }

            result.push(level);
            stack = next_level;
        }

        result
    }
}
