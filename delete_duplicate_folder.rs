/*
1948. Delete Duplicate Folders in System

Due to a bug, there are many duplicate folders in a file system. You are given a 2D array paths, where paths[i] is an array representing an absolute path to the ith folder in the file system.

For example, ["one", "two", "three"] represents the path "/one/two/three".
Two folders (not necessarily on the same level) are identical if they contain the same non-empty set of identical subfolders and underlying subfolder structure. The folders do not need to be at the root level to be identical. If two or more folders are identical, then mark the folders as well as all their subfolders.

For example, folders "/a" and "/b" in the file structure below are identical. They (as well as their subfolders) should all be marked:
/a
/a/x
/a/x/y
/a/z
/b
/b/x
/b/x/y
/b/z
However, if the file structure also included the path "/b/w", then the folders "/a" and "/b" would not be identical. Note that "/a/x" and "/b/x" would still be considered identical even with the added folder.
Once all the identical folders and their subfolders have been marked, the file system will delete all of them. The file system only runs the deletion once, so any folders that become identical after the initial deletion are not deleted.

Return the 2D array ans containing the paths of the remaining folders after deleting all the marked folders. The paths may be returned in any order.

*/

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type NodeRef = Rc<RefCell<TrieNode>>;

#[derive(Default)]
struct TrieNode {
    children: HashMap<String, NodeRef>,
    deleted: bool,
}


impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let root: NodeRef = Rc::new(RefCell::new(TrieNode::default()));

        // Build the Trie
        for path in paths.iter() {
            let mut node = Rc::clone(&root);
            for name in path {
                let next = {
                    let mut node_ref = node.borrow_mut();
                    node_ref
                        .children
                        .entry(name.clone())
                        .or_insert_with(|| Rc::new(RefCell::new(TrieNode::default())))
                        .clone()
                };
                node = next;
            }
        }

        // Map serialized subtree to nodes
        let mut subtree_to_nodes: HashMap<String, Vec<NodeRef>> = HashMap::new();
        Solution::build_subtree(&root, &mut subtree_to_nodes);

        // Mark duplicates as deleted
        for nodes in subtree_to_nodes.values() {
            if nodes.len() > 1 {
                for node in nodes {
                    node.borrow_mut().deleted = true;
                }
            }
        }

        // Construct result paths
        let mut result = vec![];
        let mut path = vec![];
        Solution::collect_paths(&root, &mut path, &mut result);
        result
    }

    fn build_subtree(
        node: &NodeRef,
        map: &mut HashMap<String, Vec<NodeRef>>,
    ) -> String {
        let mut children = node.borrow().children.clone();
        let mut keys: Vec<String> = children.keys().cloned().collect();
        keys.sort();

        let mut subtree = String::from("(");
        for key in keys {
            let child = children.get(&key).unwrap();
            subtree.push_str(&key);
            subtree.push_str(&Self::build_subtree(child, map));
        }
        subtree.push(')');

        if subtree != "()" {
            map.entry(subtree.clone()).or_default().push(Rc::clone(node));
        }

        subtree
    }

    fn collect_paths(
        node: &NodeRef,
        path: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
    ) {
        let children = node.borrow().children.clone();
        for (name, child) in children {
            if !child.borrow().deleted {
                path.push(name.clone());
                Self::collect_paths(&child, path, result);
                result.push(path.clone());
                path.pop();
            }
        }
    }
}
