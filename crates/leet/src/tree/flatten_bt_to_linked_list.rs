/// lc 114 - Flatten Binary Tree to Linked List
use std::cell::RefCell;
use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut cur = root.clone();
        while let Some(node) = cur {
            let left = node.borrow().left.clone();
            if let Some(left_node) = left {
                // Find the rightmost node of the left subtree
                let mut rightmost = left_node.clone();
                loop {
                    let next = rightmost.borrow().right.clone();
                    match next {
                        Some(n) => rightmost = n,
                        None => break,
                    }
                }
                // Link rightmost to current node's right
                let right = node.borrow().right.clone();
                rightmost.borrow_mut().right = right;
                // Move left subtree to right
                node.borrow_mut().right = Some(left_node);
                node.borrow_mut().left = None;
            }
            cur = node.borrow().right.clone();
        }
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
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;
        while i < vals.len() {
            let node = queue.pop_front().unwrap();
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().left = Some(left.clone());
                    queue.push_back(left);
                }
            }
            i += 1;
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().right = Some(right.clone());
                    queue.push_back(right);
                }
            }
            i += 1;
        }
        Some(root)
    }

    fn collect_list(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut cur = root.clone();
        while let Some(node) = cur {
            result.push(node.borrow().val);
            assert!(node.borrow().left.is_none());
            cur = node.borrow().right.clone();
        }
        result
    }

    #[test]
    fn test_example1() {
        let mut root = build_tree(&[Some(1), Some(2), Some(5), Some(3), Some(4), None, Some(6)]);
        Solution::flatten(&mut root);
        assert_eq!(collect_list(&root), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_empty() {
        let mut root = None;
        Solution::flatten(&mut root);
        assert_eq!(collect_list(&root), vec![]);
    }

    #[test]
    fn test_single() {
        let mut root = build_tree(&[Some(0)]);
        Solution::flatten(&mut root);
        assert_eq!(collect_list(&root), vec![0]);
    }
}
