/// LeetCode 103: Binary Tree Zigzag Level Order Traversal
///
/// Given the root of a binary tree, return the zigzag level order traversal of its nodes' values
/// (i.e., from left to right, then right to left for the next level, and alternate between).

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    /// BFS queue-based zigzag level order traversal.
    /// Even levels left-to-right, odd levels right-to-left.
    /// Time O(n), Space O(w) where w is max width.
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

        if let Some(node) = root {
            queue.push_back(node);
        }

        let mut left_to_right = true;

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut level = vec![0; level_size];
            for i in 0..level_size {
                let node = queue.pop_front().unwrap();
                let borrowed = node.borrow();
                let idx = if left_to_right { i } else { level_size - 1 - i };
                level[idx] = borrowed.val;
                if let Some(ref left) = borrowed.left {
                    queue.push_back(left.clone());
                }
                if let Some(ref right) = borrowed.right {
                    queue.push_back(right.clone());
                }
            }
            result.push(level);
            left_to_right = !left_to_right;
        }

        result
    }

    /// DFS recursive with depth tracking, then reverse odd levels.
    /// Time O(n), Space O(h) where h is tree height.
    pub fn zigzag_level_order_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        Self::dfs(&root, 0, &mut result);
        for (i, level) in result.iter_mut().enumerate() {
            if i % 2 == 1 {
                level.reverse();
            }
        }
        result
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, depth: usize, result: &mut Vec<Vec<i32>>) {
        if let Some(n) = node {
            let borrowed = n.borrow();
            if depth == result.len() {
                result.push(Vec::new());
            }
            result[depth].push(borrowed.val);
            Self::dfs(&borrowed.left, depth + 1, result);
            Self::dfs(&borrowed.right, depth + 1, result);
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

    /// Tree: [3,9,20,null,null,15,7]
    ///       3
    ///      / \
    ///     9  20
    ///       /  \
    ///      15   7
    #[test]
    fn test_example_1() {
        let root = node(3, leaf(9), node(20, leaf(15), leaf(7)));
        let expected = vec![vec![3], vec![20, 9], vec![15, 7]];
        assert_eq!(Solution::zigzag_level_order(root.clone()), expected);
        assert_eq!(Solution::zigzag_level_order_dfs(root), expected);
    }

    /// Tree: [1]
    #[test]
    fn test_single_node() {
        let root = leaf(1);
        let expected = vec![vec![1]];
        assert_eq!(Solution::zigzag_level_order(root.clone()), expected);
        assert_eq!(Solution::zigzag_level_order_dfs(root), expected);
    }

    #[test]
    fn test_empty() {
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::zigzag_level_order(None), expected);
        assert_eq!(Solution::zigzag_level_order_dfs(None), expected);
    }

    /// Complete tree:
    ///        1
    ///      /   \
    ///     2     3
    ///    / \   / \
    ///   4   5 6   7
    #[test]
    fn test_complete_tree() {
        let root = node(1, node(2, leaf(4), leaf(5)), node(3, leaf(6), leaf(7)));
        let expected = vec![vec![1], vec![3, 2], vec![4, 5, 6, 7]];
        assert_eq!(Solution::zigzag_level_order(root.clone()), expected);
        assert_eq!(Solution::zigzag_level_order_dfs(root), expected);
    }

    /// Four levels sparse:
    ///        1
    ///      /   \
    ///     2     3
    ///      \   /
    ///       5 4
    ///      /   \
    ///     6     7
    #[test]
    fn test_four_levels_sparse() {
        let root = node(
            1,
            node(2, None, node(5, leaf(6), None)),
            node(3, node(4, None, leaf(7)), None),
        );
        let expected = vec![vec![1], vec![3, 2], vec![5, 4], vec![7, 6]];
        assert_eq!(Solution::zigzag_level_order(root.clone()), expected);
        assert_eq!(Solution::zigzag_level_order_dfs(root), expected);
    }

    /// Negative values: [-100, 0, 100]
    #[test]
    fn test_negative_values() {
        let root = node(-100, leaf(0), leaf(100));
        let expected = vec![vec![-100], vec![100, 0]];
        assert_eq!(Solution::zigzag_level_order(root.clone()), expected);
        assert_eq!(Solution::zigzag_level_order_dfs(root), expected);
    }
}
