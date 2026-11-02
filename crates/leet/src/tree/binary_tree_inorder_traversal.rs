use std::cell::RefCell;
use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    /// Visits each node once with an explicit stack.
    /// Time O(n), Space O(h), where h is the tree height.
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut current = root;

        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                current = node.borrow().left.clone();
                stack.push(node);
            }

            let node = stack.pop().expect("stack contains the next inorder node");
            result.push(node.borrow().val);
            current = node.borrow().right.clone();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn returns_empty_for_an_empty_tree() {
        assert_eq!(Solution::inorder_traversal(None), Vec::<i32>::new());
    }

    #[test]
    fn returns_the_value_of_a_single_node() {
        assert_eq!(Solution::inorder_traversal(node(7, None, None)), vec![7]);
    }

    #[test]
    fn visits_a_balanced_tree_left_root_right() {
        let root = node(
            4,
            node(2, node(1, None, None), node(3, None, None)),
            node(6, node(5, None, None), node(7, None, None)),
        );

        assert_eq!(Solution::inorder_traversal(root), vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn visits_an_unbalanced_tree_left_root_right() {
        let root = node(1, None, node(2, node(3, None, None), None));

        assert_eq!(Solution::inorder_traversal(root), vec![1, 3, 2]);
    }
}
