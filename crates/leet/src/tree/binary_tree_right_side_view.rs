/// LeetCode 199: Binary Tree Right Side View
///
/// Given a binary tree, return the values of the nodes visible from the right side (top to bottom).

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    /// BFS level-order traversal: take the last node of each level.
    /// Time O(n), Space O(w) where w is max width.
    pub fn right_side_view_bfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

        if let Some(node) = root {
            queue.push_back(node);
        }

        while !queue.is_empty() {
            let level_size = queue.len();
            for i in 0..level_size {
                let node = queue.pop_front().unwrap();
                let borrowed = node.borrow();
                if i == level_size - 1 {
                    result.push(borrowed.val);
                }
                if let Some(ref left) = borrowed.left {
                    queue.push_back(left.clone());
                }
                if let Some(ref right) = borrowed.right {
                    queue.push_back(right.clone());
                }
            }
        }

        result
    }

    /// DFS preorder (root -> right -> left): first node at each new depth is the rightmost.
    /// Time O(n), Space O(h) where h is tree height.
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        Self::dfs(&root, 0, &mut result);
        result
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, depth: usize, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let borrowed = n.borrow();
            if depth == result.len() {
                result.push(borrowed.val);
            }
            Self::dfs(&borrowed.right, depth + 1, result);
            Self::dfs(&borrowed.left, depth + 1, result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    fn leaf(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        node(val, None, None)
    }

    /// Tree:
    ///       1
    ///      / \
    ///     2   3
    ///      \   \
    ///       5   4
    #[test]
    fn test_example_1() {
        let root = node(1, node(2, None, leaf(5)), node(3, None, leaf(4)));
        assert_eq!(Solution::right_side_view(root.clone()), vec![1, 3, 4]);
        assert_eq!(Solution::right_side_view_bfs(root), vec![1, 3, 4]);
    }

    /// Tree:
    ///     1
    ///      \
    ///       3
    #[test]
    fn test_example_2() {
        let root = node(1, None, leaf(3));
        assert_eq!(Solution::right_side_view(root.clone()), vec![1, 3]);
        assert_eq!(Solution::right_side_view_bfs(root), vec![1, 3]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::right_side_view(None), Vec::<i32>::new());
        assert_eq!(Solution::right_side_view_bfs(None), Vec::<i32>::new());
    }

    /// Tree:
    ///       1
    ///      /
    ///     2
    ///    /
    ///   3
    #[test]
    fn test_left_skewed() {
        let root = node(1, node(2, leaf(3), None), None);
        assert_eq!(Solution::right_side_view(root.clone()), vec![1, 2, 3]);
        assert_eq!(Solution::right_side_view_bfs(root), vec![1, 2, 3]);
    }
}
