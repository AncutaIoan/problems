/*
99. Recover Binary Search Tree

You are given the root of a binary search tree (BST),
where the values of exactly two nodes of the tree were swapped by mistake. 
Recover the tree without changing its structure.


Can also be done by using inorder of the graph and checking the change values
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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let (mut first, mut second, mut prev) = (None, None, None);
        Solution::inorder(root, &mut first, &mut second, &mut prev);
        
        // Swap values to fix the tree
        if let (Some(f), Some(s)) = (first, second) {
            let mut f_mut = f.borrow_mut();
            let mut s_mut = s.borrow_mut();
            std::mem::swap(&mut f_mut.val, &mut s_mut.val);
        }
    }

    fn inorder(
        node: &Option<Rc<RefCell<TreeNode>>>,
        first: &mut Option<Rc<RefCell<TreeNode>>>,
        second: &mut Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(n_rc) = node {
            let n_ref = n_rc.borrow();
            
            // Recursive call to left subtree
            Solution::inorder(&n_ref.left, first, second, prev);
            
            // Detect swapped nodes
            if let Some(prev_rc) = prev {
                let prev_ref = prev_rc.borrow();
                if prev_ref.val > n_ref.val {
                    if first.is_none() {
                        *first = Some(Rc::clone(prev_rc)); // First misplaced node
                    }
                    *second = Some(Rc::clone(n_rc)); // Second misplaced node
                }
            }
            
            // Update prev node to current node
            *prev = Some(Rc::clone(n_rc));

            // Recursive call to right subtree
            Solution::inorder(&n_ref.right, first, second, prev);
        }
    }
}
