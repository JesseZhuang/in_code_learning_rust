use std::cell::RefCell;
/// lc 98 - Validate Binary Search Tree
use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    /// Recursive approach with i64 bounds.
    /// O(n) time — visits every node once.
    /// O(h) space — recursion stack depth equals tree height.
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::validate(&root, i64::MIN, i64::MAX)
    }

    fn validate(node: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        match node {
            None => true,
            Some(n) => {
                let n = n.borrow();
                let val = n.val as i64;
                if val <= min || val >= max {
                    return false; // O(1) bound check
                }
                Self::validate(&n.left, min, val) && Self::validate(&n.right, val, max)
            }
        }
    }

    /// Iterative inorder traversal with explicit stack.
    /// O(n) time — visits every node once.
    /// O(h) space — stack holds at most h nodes.
    pub fn is_valid_bst_inorder(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut current = root;
        let mut prev = i64::MIN; // track previous inorder value

        loop {
            while let Some(node) = current {
                stack.push(node.clone());
                current = node.borrow().left.clone(); // O(h) stack growth
            }
            if let Some(node) = stack.pop() {
                let val = node.borrow().val as i64;
                if val <= prev {
                    return false; // inorder must be strictly increasing
                }
                prev = val;
                current = node.borrow().right.clone();
            } else {
                break;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn test_valid_bst() {
        // [2,1,3] -> true
        let tree = node(2, node(1, None, None), node(3, None, None));
        assert!(Solution::is_valid_bst(tree.clone()));
        assert!(Solution::is_valid_bst_inorder(tree));
    }

    #[test]
    fn test_invalid_bst() {
        // [5,1,4,null,null,3,6] -> false
        let tree = node(5,
            node(1, None, None),
            node(4, node(3, None, None), node(6, None, None)),
        );
        assert!(!Solution::is_valid_bst(tree.clone()));
        assert!(!Solution::is_valid_bst_inorder(tree));
    }

    #[test]
    fn test_single_node() {
        let tree = node(1, None, None);
        assert!(Solution::is_valid_bst(tree.clone()));
        assert!(Solution::is_valid_bst_inorder(tree));
    }

    #[test]
    fn test_equal_values() {
        // [2,2,2] -> false (BST requires strict inequality)
        let tree = node(2, node(2, None, None), node(2, None, None));
        assert!(!Solution::is_valid_bst(tree.clone()));
        assert!(!Solution::is_valid_bst_inorder(tree));
    }

    #[test]
    fn test_int_max_node() {
        // Single node with i32::MAX should be valid
        let tree = node(i32::MAX, None, None);
        assert!(Solution::is_valid_bst(tree.clone()));
        assert!(Solution::is_valid_bst_inorder(tree));
    }

    #[test]
    fn test_left_skewed() {
        // 3 -> 2 -> 1 (left skewed, valid BST)
        let tree = node(3, node(2, node(1, None, None), None), None);
        assert!(Solution::is_valid_bst(tree.clone()));
        assert!(Solution::is_valid_bst_inorder(tree));
    }

    #[test]
    fn test_right_skewed() {
        // 1 -> 2 -> 3 (right skewed, valid BST)
        let tree = node(1, None, node(2, None, node(3, None, None)));
        assert!(Solution::is_valid_bst(tree.clone()));
        assert!(Solution::is_valid_bst_inorder(tree));
    }
}
