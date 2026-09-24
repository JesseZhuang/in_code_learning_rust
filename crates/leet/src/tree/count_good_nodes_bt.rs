use std::cell::RefCell;
use std::collections::VecDeque;
/// lc 1448
use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    /// DFS recursive — track max value from root to current node.
    /// O(n) time, O(h) space.
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, max_so_far: i32) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    let good = if n.val >= max_so_far { 1 } else { 0 };
                    let new_max = max_so_far.max(n.val);
                    good + dfs(n.left.clone(), new_max) + dfs(n.right.clone(), new_max)
                }
            }
        }
        dfs(root, i32::MIN)
    }

    /// BFS iterative — queue stores (node, max_so_far) pairs.
    /// O(n) time, O(w) space where w is max tree width.
    pub fn good_nodes_bfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = 0;
        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();
        if let Some(r) = root {
            queue.push_back((r, i32::MIN));
        }
        while let Some((node, max_so_far)) = queue.pop_front() {
            let n = node.borrow();
            if n.val >= max_so_far {
                count += 1;
            }
            let new_max = max_so_far.max(n.val);
            if let Some(ref left) = n.left {
                queue.push_back((Rc::clone(left), new_max));
            }
            if let Some(ref right) = n.right {
                queue.push_back((Rc::clone(right), new_max));
            }
        }
        count
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
        let opts: Vec<Option<i32>> = vals.iter()
            .map(|&v| if v == i32::MIN { None } else { Some(v) })
            .collect();
        build_tree(&opts)
    }

    const N: i32 = i32::MIN;

    #[test]
    fn test_example1() {
        // [3,1,4,3,null,1,5] -> 4 good nodes: 3, 3, 4, 5
        let t = tree(&[3, 1, 4, 3, N, 1, 5]);
        assert_eq!(Solution::good_nodes(t.clone()), 4);
        assert_eq!(Solution::good_nodes_bfs(t), 4);
    }

    #[test]
    fn test_example2() {
        // [3,3,null,4,2] -> 3 good nodes: 3, 3, 4
        let t = tree(&[3, 3, N, 4, 2]);
        assert_eq!(Solution::good_nodes(t.clone()), 3);
        assert_eq!(Solution::good_nodes_bfs(t), 3);
    }

    #[test]
    fn test_single_node() {
        let t = tree(&[1]);
        assert_eq!(Solution::good_nodes(t.clone()), 1);
        assert_eq!(Solution::good_nodes_bfs(t), 1);
    }

    #[test]
    fn test_all_equal() {
        // [5,5,5] -> all 3 are good
        let t = tree(&[5, 5, 5]);
        assert_eq!(Solution::good_nodes(t.clone()), 3);
        assert_eq!(Solution::good_nodes_bfs(t), 3);
    }

    #[test]
    fn test_strictly_decreasing() {
        // [10,5,null,3,null,1] -> only root is good
        let t = tree(&[10, 5, N, 3, N, 1]);
        assert_eq!(Solution::good_nodes(t.clone()), 1);
        assert_eq!(Solution::good_nodes_bfs(t), 1);
    }

    #[test]
    fn test_right_chain_increasing() {
        // [1,null,2,null,3,null,4] -> all 4 good
        let t = tree(&[1, N, 2, N, 3, N, 4]);
        assert_eq!(Solution::good_nodes(t.clone()), 4);
        assert_eq!(Solution::good_nodes_bfs(t), 4);
    }

    #[test]
    fn test_negative_values() {
        // [-1,-2,-3] -> only root is good (-1 >= -1, -2 < -1, -3 < -1)
        let t = tree(&[-1, -2, -3]);
        assert_eq!(Solution::good_nodes(t.clone()), 1);
        assert_eq!(Solution::good_nodes_bfs(t), 1);
    }

    #[test]
    fn test_negative_increasing() {
        // [-10,-5,-10] -> -10 good (root), -5 good (-5 >= -10), -10 good (-10 >= -10)
        let t = tree(&[-10, -5, -10]);
        assert_eq!(Solution::good_nodes(t.clone()), 3);
        assert_eq!(Solution::good_nodes_bfs(t), 3);
    }
}
