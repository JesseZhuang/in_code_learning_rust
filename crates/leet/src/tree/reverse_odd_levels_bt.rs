/// leet 2415

use std::cell::RefCell;
use std::rc::Rc;

type TreeNodeRef = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: TreeNodeRef,
    pub right: TreeNodeRef,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

pub struct Solution;

impl Solution {
    /// DFS. O(n) time, O(log n) space.
    pub fn reverse_odd_levels(root: TreeNodeRef) -> TreeNodeRef {
        if let Some(ref node) = root {
            let node = node.borrow();
            Self::dfs(&node.left, &node.right, 1);
        }
        root
    }

    fn dfs(l: &TreeNodeRef, r: &TreeNodeRef, d: i32) {
        match (l, r) {
            (Some(left), Some(right)) => {
                if d % 2 == 1 {
                    let mut lb = left.borrow_mut();
                    let mut rb = right.borrow_mut();
                    std::mem::swap(&mut lb.val, &mut rb.val);
                }
                let lb = left.borrow();
                let rb = right.borrow();
                Self::dfs(&lb.left, &rb.right, d + 1);
                Self::dfs(&lb.right, &rb.left, d + 1);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn build_tree(vals: &[i32]) -> TreeNodeRef {
        if vals.is_empty() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(vals[0])));
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));
        let mut i = 1;
        while i < vals.len() {
            let node = queue.pop_front().unwrap();
            let mut n = node.borrow_mut();
            if i < vals.len() {
                let left = Rc::new(RefCell::new(TreeNode::new(vals[i])));
                queue.push_back(Rc::clone(&left));
                n.left = Some(left);
                i += 1;
            }
            if i < vals.len() {
                let right = Rc::new(RefCell::new(TreeNode::new(vals[i])));
                queue.push_back(Rc::clone(&right));
                n.right = Some(right);
                i += 1;
            }
        }
        Some(root)
    }

    fn level_order(root: &TreeNodeRef) -> Vec<i32> {
        let mut res = Vec::new();
        let mut queue = VecDeque::new();
        if let Some(ref node) = root {
            queue.push_back(Rc::clone(node));
        }
        while let Some(node) = queue.pop_front() {
            let n = node.borrow();
            res.push(n.val);
            if let Some(ref left) = n.left {
                queue.push_back(Rc::clone(left));
            }
            if let Some(ref right) = n.right {
                queue.push_back(Rc::clone(right));
            }
        }
        res
    }

    #[test]
    fn test_reverse_odd_levels_example1() {
        let root = build_tree(&[2, 3, 5, 8, 13, 21, 34]);
        let result = Solution::reverse_odd_levels(root);
        assert_eq!(level_order(&result), vec![2, 5, 3, 8, 13, 21, 34]);
    }

    #[test]
    fn test_reverse_odd_levels_example2() {
        let root = build_tree(&[7, 13, 11]);
        let result = Solution::reverse_odd_levels(root);
        assert_eq!(level_order(&result), vec![7, 11, 13]);
    }

    #[test]
    fn test_reverse_odd_levels_example3() {
        let root = build_tree(&[0, 1, 2, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2]);
        let result = Solution::reverse_odd_levels(root);
        assert_eq!(level_order(&result), vec![0, 2, 1, 0, 0, 0, 0, 2, 2, 2, 2, 1, 1, 1, 1]);
    }
}
