use std::cell::RefCell;
use std::collections::VecDeque;
/// lc 226
use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    /// Recursive DFS: swap left and right children, then recurse.
    /// O(n) time, O(h) space (call stack depth = tree height).
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut n = node.borrow_mut();
            let left = n.left.take();
            let right = n.right.take();
            n.left = Self::invert_tree(right);
            n.right = Self::invert_tree(left);
            drop(n);
            Some(node)
        } else {
            None
        }
    }

    /// Iterative BFS using VecDeque: level-by-level swap.
    /// O(n) time, O(n) space (queue holds up to n/2 nodes at widest level).
    pub fn invert_tree_bfs(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref r) = root {
            let mut queue = VecDeque::new();
            queue.push_back(Rc::clone(r));
            while let Some(node) = queue.pop_front() {
                let mut n = node.borrow_mut();
                // swap children
                let tmp = n.left.take();
                n.left = n.right.take();
                n.right = tmp;
                // enqueue children for next level
                if let Some(ref left) = n.left {
                    queue.push_back(Rc::clone(left));
                }
                if let Some(ref right) = n.right {
                    queue.push_back(Rc::clone(right));
                }
            }
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));
        let mut i = 1;
        while i < vals.len() {
            let node = queue.pop_front().unwrap();
            let mut n = node.borrow_mut();
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(v)));
                    queue.push_back(Rc::clone(&left));
                    n.left = Some(left);
                }
                i += 1;
            }
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(v)));
                    queue.push_back(Rc::clone(&right));
                    n.right = Some(right);
                }
                i += 1;
            }
        }
        Some(root)
    }

    fn tree(vals: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let opts: Vec<Option<i32>> = vals
            .iter()
            .map(|&v| if v == i32::MIN { None } else { Some(v) })
            .collect();
        build_tree(&opts)
    }

    /// Convert tree back to level-order vec (standard LeetCode serialization).
    fn tree_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut result = Vec::new();
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(root.clone());
        while let Some(item) = queue.pop_front() {
            match item {
                Some(node) => {
                    let n = node.borrow();
                    result.push(Some(n.val));
                    queue.push_back(n.left.clone());
                    queue.push_back(n.right.clone());
                }
                None => {
                    result.push(None);
                }
            }
        }
        // trim trailing Nones
        while result.last() == Some(&None) {
            result.pop();
        }
        result
    }

    fn to_opts(vals: &[i32]) -> Vec<Option<i32>> {
        vals.iter()
            .map(|&v| if v == i32::MIN { None } else { Some(v) })
            .collect()
    }

    // ---- Recursive DFS tests ----

    #[test]
    fn test_example1() {
        // [4,2,7,1,3,6,9] -> [4,7,2,9,6,3,1]
        let root = tree(&[4, 2, 7, 1, 3, 6, 9]);
        let inverted = Solution::invert_tree(root);
        assert_eq!(tree_to_vec(&inverted), to_opts(&[4, 7, 2, 9, 6, 3, 1]));
    }

    #[test]
    fn test_example2() {
        // [2,1,3] -> [2,3,1]
        let root = tree(&[2, 1, 3]);
        let inverted = Solution::invert_tree(root);
        assert_eq!(tree_to_vec(&inverted), to_opts(&[2, 3, 1]));
    }

    #[test]
    fn test_none() {
        assert_eq!(tree_to_vec(&Solution::invert_tree(None)), Vec::<Option<i32>>::new());
    }

    #[test]
    fn test_single_node() {
        let root = tree(&[1]);
        let inverted = Solution::invert_tree(root);
        assert_eq!(tree_to_vec(&inverted), to_opts(&[1]));
    }

    #[test]
    fn test_left_only() {
        // Left-only tree: 1 -> 2 -> 3 becomes right-only: 1 -> 2 -> 3
        let n = i32::MIN;
        let root = tree(&[1, 2, n, 3]);
        let inverted = Solution::invert_tree(root);
        assert_eq!(
            tree_to_vec(&inverted),
            to_opts(&[1, n, 2, n, 3])
        );
    }

    #[test]
    fn test_asymmetric() {
        //       1
        //      / \
        //     2   3
        //    /     \
        //   4       5
        // Inverted:
        //       1
        //      / \
        //     3   2
        //    /     \
        //   5       4
        let n = i32::MIN;
        let root = tree(&[1, 2, 3, 4, n, n, 5]);
        let inverted = Solution::invert_tree(root);
        assert_eq!(
            tree_to_vec(&inverted),
            to_opts(&[1, 3, 2, 5, n, n, 4])
        );
    }

    // ---- Iterative BFS tests ----

    #[test]
    fn test_bfs_example1() {
        let root = tree(&[4, 2, 7, 1, 3, 6, 9]);
        let inverted = Solution::invert_tree_bfs(root);
        assert_eq!(tree_to_vec(&inverted), to_opts(&[4, 7, 2, 9, 6, 3, 1]));
    }

    #[test]
    fn test_bfs_example2() {
        let root = tree(&[2, 1, 3]);
        let inverted = Solution::invert_tree_bfs(root);
        assert_eq!(tree_to_vec(&inverted), to_opts(&[2, 3, 1]));
    }

    #[test]
    fn test_bfs_none() {
        assert_eq!(
            tree_to_vec(&Solution::invert_tree_bfs(None)),
            Vec::<Option<i32>>::new()
        );
    }

    #[test]
    fn test_bfs_single_node() {
        let root = tree(&[1]);
        let inverted = Solution::invert_tree_bfs(root);
        assert_eq!(tree_to_vec(&inverted), to_opts(&[1]));
    }

    #[test]
    fn test_bfs_left_only() {
        let n = i32::MIN;
        let root = tree(&[1, 2, n, 3]);
        let inverted = Solution::invert_tree_bfs(root);
        assert_eq!(
            tree_to_vec(&inverted),
            to_opts(&[1, n, 2, n, 3])
        );
    }

    #[test]
    fn test_bfs_asymmetric() {
        let n = i32::MIN;
        let root = tree(&[1, 2, 3, 4, n, n, 5]);
        let inverted = Solution::invert_tree_bfs(root);
        assert_eq!(
            tree_to_vec(&inverted),
            to_opts(&[1, 3, 2, 5, n, n, 4])
        );
    }
}
