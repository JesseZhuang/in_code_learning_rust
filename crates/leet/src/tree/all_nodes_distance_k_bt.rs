use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

/// lc 863 — All Nodes Distance K in Binary Tree

pub struct Solution;

impl Solution {
    /// BFS with parent map approach.
    /// 1. DFS to build a parent pointer map (child_val -> parent_val).
    /// 2. BFS from target node for k levels using adjacency (left, right, parent).
    /// Time O(n), Space O(n).
    pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: i32, k: i32) -> Vec<i32> {
        // Build adjacency list: val -> vec of neighbor vals
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

        fn build_graph(node: &Option<Rc<RefCell<TreeNode>>>, graph: &mut HashMap<i32, Vec<i32>>) {
            if let Some(n) = node {
                let n = n.borrow();
                if let Some(ref left) = n.left {
                    let lv = left.borrow().val;
                    graph.entry(n.val).or_default().push(lv);
                    graph.entry(lv).or_default().push(n.val);
                    build_graph(&n.left, graph);
                }
                if let Some(ref right) = n.right {
                    let rv = right.borrow().val;
                    graph.entry(n.val).or_default().push(rv);
                    graph.entry(rv).or_default().push(n.val);
                    build_graph(&n.right, graph);
                }
            }
        }

        build_graph(&root, &mut graph);

        // BFS from target for k levels
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(target);
        visited.insert(target);
        let mut dist = 0;

        while !queue.is_empty() {
            if dist == k {
                return queue.into_iter().collect();
            }
            let size = queue.len();
            for _ in 0..size {
                let curr = queue.pop_front().unwrap();
                if let Some(neighbors) = graph.get(&curr) {
                    for &nei in neighbors {
                        if visited.insert(nei) {
                            queue.push_back(nei);
                        }
                    }
                }
            }
            dist += 1;
        }

        vec![]
    }

    /// Pure DFS approach.
    /// Find target recursively; once found, collect nodes at remaining distance in subtrees.
    /// Time O(n), Space O(n).
    pub fn distance_k_dfs(root: Option<Rc<RefCell<TreeNode>>>, target: i32, k: i32) -> Vec<i32> {
        let mut result = Vec::new();

        /// Collect all nodes at distance `dist` below `node`.
        fn collect_subtree(node: &Option<Rc<RefCell<TreeNode>>>, dist: i32, result: &mut Vec<i32>) {
            if dist < 0 {
                return;
            }
            if let Some(n) = node {
                let n = n.borrow();
                if dist == 0 {
                    result.push(n.val);
                    return;
                }
                collect_subtree(&n.left, dist - 1, result);
                collect_subtree(&n.right, dist - 1, result);
            }
        }

        /// Returns the distance from `node` to target if target is in this subtree, else -1.
        fn dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            target: i32,
            k: i32,
            result: &mut Vec<i32>,
        ) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();
                if n.val == target {
                    // Collect nodes at distance k in target's subtree
                    collect_subtree(&Some(Rc::new(RefCell::new(TreeNode {
                        val: n.val,
                        left: n.left.clone(),
                        right: n.right.clone(),
                    }))), k, result);
                    return 0;
                }

                let left_dist = dfs(&n.left, target, k, result);
                if left_dist >= 0 {
                    // Target is in left subtree at distance left_dist
                    let dist_from_here = left_dist + 1;
                    if dist_from_here == k {
                        result.push(n.val);
                    } else if dist_from_here < k {
                        // Look in right subtree for nodes at remaining distance
                        collect_subtree(&n.right, k - dist_from_here - 1, result);
                    }
                    return dist_from_here;
                }

                let right_dist = dfs(&n.right, target, k, result);
                if right_dist >= 0 {
                    let dist_from_here = right_dist + 1;
                    if dist_from_here == k {
                        result.push(n.val);
                    } else if dist_from_here < k {
                        collect_subtree(&n.left, k - dist_from_here - 1, result);
                    }
                    return dist_from_here;
                }

                -1 // target not in this subtree
            } else {
                -1
            }
        }

        dfs(&root, target, k, &mut result);
        result
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

    fn sorted(mut v: Vec<i32>) -> Vec<i32> {
        v.sort();
        v
    }

    //       3
    //      / \
    //     5   1
    //    / \ / \
    //   6  2 0  8
    //     / \
    //    7   4
    // target=5, k=2 -> [7, 4, 1]
    #[test]
    fn test_bfs_example1() {
        let n = i32::MIN;
        let root = tree(&[3, 5, 1, 6, 2, 0, 8, n, n, 7, 4]);
        assert_eq!(sorted(Solution::distance_k(root, 5, 2)), vec![1, 4, 7]);
    }

    #[test]
    fn test_bfs_target_is_root_k0() {
        let root = tree(&[1, 2, 3]);
        assert_eq!(sorted(Solution::distance_k(root, 1, 0)), vec![1]);
    }

    #[test]
    fn test_bfs_single_node() {
        let root = tree(&[1]);
        assert_eq!(Solution::distance_k(root.clone(), 1, 0), vec![1]);
        assert_eq!(Solution::distance_k(root, 1, 1), vec![]);
    }

    #[test]
    fn test_bfs_k_larger_than_tree() {
        let root = tree(&[1, 2, 3]);
        assert_eq!(Solution::distance_k(root, 1, 5), vec![]);
    }

    #[test]
    fn test_dfs_example1() {
        let n = i32::MIN;
        let root = tree(&[3, 5, 1, 6, 2, 0, 8, n, n, 7, 4]);
        assert_eq!(sorted(Solution::distance_k_dfs(root, 5, 2)), vec![1, 4, 7]);
    }

    #[test]
    fn test_dfs_target_is_root_k0() {
        let root = tree(&[1, 2, 3]);
        assert_eq!(sorted(Solution::distance_k_dfs(root, 1, 0)), vec![1]);
    }

    #[test]
    fn test_dfs_single_node() {
        let root = tree(&[1]);
        assert_eq!(Solution::distance_k_dfs(root.clone(), 1, 0), vec![1]);
        assert_eq!(Solution::distance_k_dfs(root, 1, 1), vec![]);
    }

    #[test]
    fn test_dfs_k_larger_than_tree() {
        let root = tree(&[1, 2, 3]);
        assert_eq!(Solution::distance_k_dfs(root, 1, 5), vec![]);
    }

    #[test]
    fn test_dfs_leaf_target() {
        // target=7 (leaf), k=3 -> should get nodes 3 levels up/across
        let n = i32::MIN;
        let root = tree(&[3, 5, 1, 6, 2, 0, 8, n, n, 7, 4]);
        // distance from 7: 7->2(1)->5(2)->3(3) and 7->2(1)->4(2) ... wait
        // 7's parent is 2, 2's parent is 5, 5's parent is 3
        // dist 1: 2, dist 2: 5,4, dist 3: 3,6
        assert_eq!(sorted(Solution::distance_k_dfs(root, 7, 3)), vec![3, 6]);
    }

    #[test]
    fn test_bfs_leaf_target() {
        let n = i32::MIN;
        let root = tree(&[3, 5, 1, 6, 2, 0, 8, n, n, 7, 4]);
        assert_eq!(sorted(Solution::distance_k(root, 7, 3)), vec![3, 6]);
    }
}
