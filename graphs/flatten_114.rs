/*

114. Flatten Binary Tree to Linked List

Given the root of a binary tree, flatten the tree into a "linked list":

The "linked list" should use the same TreeNode class where the right child pointer points to the next node in the list and the left child pointer is always null.
The "linked list" should be in the same order as a pre-order traversal of the binary tree.


*/


use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn helper(node: &mut Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<Rc<RefCell<TreeNode>>>) {
            if let Some(n_rc) = node.clone() {
                let mut n = n_rc.borrow_mut();

                // Process the right subtree first (postorder)
                let right = n.right.take();
                let left = n.left.take();
                
                helper(&mut right.clone(), prev);
                helper(&mut left.clone(), prev);

                // Flattening
                n.right = prev.clone();
                n.left = None;

                // Move the previous pointer to the current node (FIX: Clone n_rc)
                *prev = Some(Rc::clone(&n_rc)); // <-- FIXED: Clone instead of moving
            }
        }

        let mut prev = None;
        helper(root, &mut prev);
    }
}
