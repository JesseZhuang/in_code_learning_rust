use std::cell::RefCell;
use std::cmp::max;
/// lc 337
use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    /// Time O(n), Space O(h) — post-order DFS returning (rob_this, skip_this).
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (rob, skip) = Self::dfs(&root);
        max(rob, skip)
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match node {
            None => (0, 0),
            Some(n) => {
                let n = n.borrow();
                let (rob_left, skip_left) = Self::dfs(&n.left);
                let (rob_right, skip_right) = Self::dfs(&n.right);
                // rob_this: take this node's value + must skip both children
                let rob_this = n.val + skip_left + skip_right;
                // skip_this: don't take this node, pick best from each child
                let skip_this = max(rob_left, skip_left) + max(rob_right, skip_right);
                (rob_this, skip_this)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

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

    #[test]
    fn test_example1() {
        // [3,2,3,null,3,null,1] -> 7 (rob 3 + 3 + 1)
        let n = i32::MIN;
        assert_eq!(Solution::rob(tree(&[3, 2, 3, n, 3, n, 1])), 7);
    }

    #[test]
    fn test_example2() {
        // [3,4,5,1,3,null,1] -> 9 (rob 4 + 5)
        let n = i32::MIN;
        assert_eq!(Solution::rob(tree(&[3, 4, 5, 1, 3, n, 1])), 9);
    }

    #[test]
    fn test_single_node() {
        assert_eq!(Solution::rob(tree(&[5])), 5);
    }

    #[test]
    fn test_none() {
        assert_eq!(Solution::rob(None), 0);
    }

    #[test]
    fn test_left_skew() {
        // [4,1,null,null,2] -> 6 (rob 4 + 2)
        let n = i32::MIN;
        assert_eq!(Solution::rob(tree(&[4, 1, n, n, 2])), 6);
    }
}
