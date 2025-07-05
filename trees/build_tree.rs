/*
105. Construct Binary Tree from Preorder and Inorder Traversal

Given two integer arrays preorder and inorder where preorder is the preorder traversal of a binary tree and inorder is the inorder traversal of the same tree, construct and return the binary tree.

 
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




impl TreeNode {
    pub fn with_children(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Self {
        TreeNode { val, left, right }
    }
}
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut inorder_indices = HashMap::new();
        for (i, &val) in inorder.iter().enumerate() {
            inorder_indices.insert(val, i);
        }

        fn build(
            preorder: &[i32],
            preorder_start: usize,
            inorder_start: usize,
            size: usize,
            inorder_indices: &HashMap<i32, usize>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if size == 0 {
                return None;
            }

            let root_val = preorder[preorder_start];
            let inorder_root_index = inorder_indices[&root_val];
            let left_size = inorder_root_index - inorder_start;

            let left = build(preorder, preorder_start + 1, inorder_start, left_size, inorder_indices);
            let right = build(
                preorder,
                preorder_start + 1 + left_size,
                inorder_root_index + 1,
                size - 1 - left_size,
                inorder_indices,
            );

            Some(Rc::new(RefCell::new(TreeNode::with_children(root_val, left, right))))
        }

        build(&preorder, 0, 0, preorder.len(), &inorder_indices)
    }
}
