use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
/// lc 662
use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    /// BFS approach: level-order traversal tracking positions.
    /// O(n) time, O(n) space (queue holds at most one level of nodes).
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else { return 0 };
        let mut max_width: u64 = 1;
        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, u64)> = VecDeque::new();
        queue.push_back((root, 0));
        while !queue.is_empty() {
            let level_size = queue.len();
            let left_pos = queue.front().unwrap().1; // leftmost position in this level
            let mut right_pos = left_pos;
            for _ in 0..level_size {
                let (node, pos) = queue.pop_front().unwrap();
                right_pos = pos;
                let normalized = pos - left_pos; // prevent overflow by normalizing
                let n = node.borrow();
                if let Some(ref left) = n.left {
                    queue.push_back((Rc::clone(left), normalized * 2)); // left child = 2*pos
                }
                if let Some(ref right) = n.right {
                    queue.push_back((Rc::clone(right), normalized * 2 + 1)); // right child = 2*pos+1
                }
            }
            max_width = max_width.max(right_pos - left_pos + 1);
        }
        max_width as i32
    }

    /// DFS approach: record first position at each depth.
    /// O(n) time, O(n) space (HashMap + recursion stack).
    pub fn width_of_binary_tree_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            depth: u32,
            pos: u64,
            first_pos: &mut HashMap<u32, u64>,
            max_width: &mut u64,
        ) {
            let Some(node) = node else { return };
            first_pos.entry(depth).or_insert(pos); // record first position at this depth
            let width = pos - first_pos[&depth] + 1;
            *max_width = (*max_width).max(width);
            let n = node.borrow();
            let normalized = pos - first_pos[&depth]; // normalize to prevent overflow
            dfs(n.left.clone(), depth + 1, normalized * 2, first_pos, max_width);
            dfs(n.right.clone(), depth + 1, normalized * 2 + 1, first_pos, max_width);
        }
        let mut first_pos = HashMap::new();
        let mut max_width: u64 = 0;
        dfs(root, 0, 0, &mut first_pos, &mut max_width);
        max_width as i32
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

    const N: i32 = i32::MIN;

    #[test]
    fn test_example1() {
        // [1,3,2,5,3,null,9] → width 4
        let t = tree(&[1, 3, 2, 5, 3, N, 9]);
        assert_eq!(Solution::width_of_binary_tree(t.clone()), 4);
        assert_eq!(Solution::width_of_binary_tree_dfs(t), 4);
    }

    #[test]
    fn test_example2() {
        // [1,3,2,5,null,null,9,6,null,7] → width 7
        // 7 is left child of 9
        let t = tree(&[1, 3, 2, 5, N, N, 9, 6, N, 7]);
        assert_eq!(Solution::width_of_binary_tree(t.clone()), 7);
        assert_eq!(Solution::width_of_binary_tree_dfs(t), 7);
    }

    #[test]
    fn test_example3() {
        // [1,3,2,5] → width 2
        let t = tree(&[1, 3, 2, 5]);
        assert_eq!(Solution::width_of_binary_tree(t.clone()), 2);
        assert_eq!(Solution::width_of_binary_tree_dfs(t), 2);
    }

    #[test]
    fn test_single_node() {
        let t = tree(&[1]);
        assert_eq!(Solution::width_of_binary_tree(t.clone()), 1);
        assert_eq!(Solution::width_of_binary_tree_dfs(t), 1);
    }

    #[test]
    fn test_left_skewed() {
        // [1,2,null,3] → width 1
        let t = tree(&[1, 2, N, 3]);
        assert_eq!(Solution::width_of_binary_tree(t.clone()), 1);
        assert_eq!(Solution::width_of_binary_tree_dfs(t), 1);
    }

    #[test]
    fn test_complete_tree() {
        // [1,2,3,4,5,6,7] → width 4
        let t = tree(&[1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(Solution::width_of_binary_tree(t.clone()), 4);
        assert_eq!(Solution::width_of_binary_tree_dfs(t), 4);
    }

    #[test]
    fn test_wide_gap() {
        // [1,2,3,4,null,null,5] → width 4
        let t = tree(&[1, 2, 3, 4, N, N, 5]);
        assert_eq!(Solution::width_of_binary_tree(t.clone()), 4);
        assert_eq!(Solution::width_of_binary_tree_dfs(t), 4);
    }
}
