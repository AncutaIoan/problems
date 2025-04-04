use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
            if let Some(n) = node {
                let left = dfs(n.borrow().left.clone());
                let right = dfs(n.borrow().right.clone());
                
                if left.0 > right.0 {
                    return (left.0 + 1, left.1);
                } else if left.0 < right.0 {
                    return (right.0 + 1, right.1);
                } else {
                    return (left.0 + 1, Some(n.clone()));
                }
            }
            (0, None)
        }
        
        dfs(root).1
    }
}
