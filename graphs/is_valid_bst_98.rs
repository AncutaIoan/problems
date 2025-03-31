/*
98. Validate Binary Search Tree


Given the root of a binary tree, determine if it is a valid binary search tree (BST).

A valid BST is defined as follows:

The left subtree of a node contains only nodes with keys less than the node's key.
The right subtree of a node contains only nodes with keys greater than the node's key.
Both the left and right subtrees must also be binary search trees.
*/

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::validate(root, None, None)
    }

    fn validate(node: Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
        if let Some(n) = node {
            let val = n.borrow().val;

            // Check if the node's value violates min/max constraints
            if let Some(min_val) = min {
                if val <= min_val {
                    return false;
                }
            }
            if let Some(max_val) = max {
                if val >= max_val {
                    return false;
                }
            }

            // Recursively validate left and right subtrees with updated constraints
            let left_valid = Self::validate(n.borrow().left.clone(), min, Some(val));
            let right_valid = Self::validate(n.borrow().right.clone(), Some(val), max);
            
            return left_valid && right_valid;
        }
        true
    }
}
