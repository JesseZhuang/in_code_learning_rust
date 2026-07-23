/// LeetCode 437 - Path Sum III
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

pub struct Solution;

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    /// DFS + prefix sum HashMap approach.
    /// Time: O(n), Space: O(n)
    pub fn path_sum(root: TreeLink, target_sum: i32) -> i32 {
        let mut prefix_map: HashMap<i64, i32> = HashMap::new();
        prefix_map.insert(0, 1);
        Self::dfs_prefix(&root, 0, target_sum as i64, &mut prefix_map)
    }

    fn dfs_prefix(
        node: &TreeLink,
        curr_sum: i64,
        target: i64,
        prefix_map: &mut HashMap<i64, i32>,
    ) -> i32 {
        let Some(n) = node else { return 0 };
        let n = n.borrow();
        let curr_sum = curr_sum + n.val as i64;
        // Count paths ending here with sum == target
        let mut count = *prefix_map.get(&(curr_sum - target)).unwrap_or(&0);

        *prefix_map.entry(curr_sum).or_insert(0) += 1;
        count += Self::dfs_prefix(&n.left, curr_sum, target, prefix_map);
        count += Self::dfs_prefix(&n.right, curr_sum, target, prefix_map);
        *prefix_map.get_mut(&curr_sum).unwrap() -= 1;

        count
    }

    /// Double DFS brute force approach.
    /// Time: O(n^2), Space: O(n) (recursion stack)
    pub fn path_sum_brute(root: TreeLink, target_sum: i32) -> i32 {
        let Some(n) = root else { return 0 };
        let nb = n.borrow();
        // Count paths starting from this node + recurse on children
        Self::count_from(&Some(n.clone()), target_sum as i64)
            + Self::path_sum_brute(nb.left.clone(), target_sum)
            + Self::path_sum_brute(nb.right.clone(), target_sum)
    }

    /// Count paths starting from `node` that sum to `target`.
    fn count_from(node: &TreeLink, target: i64) -> i32 {
        let Some(n) = node else { return 0 };
        let n = n.borrow();
        let remaining = target - n.val as i64;
        let mut count = if remaining == 0 { 1 } else { 0 };
        count += Self::count_from(&n.left, remaining);
        count += Self::count_from(&n.right, remaining);
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to build a tree from a level-order array (None = null).
    fn build_tree(vals: &[Option<i32>]) -> TreeLink {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;
        while i < vals.len() {
            let curr = queue.pop_front().unwrap();
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(v)));
                    curr.borrow_mut().left = Some(left.clone());
                    queue.push_back(left);
                }
                i += 1;
            }
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(v)));
                    curr.borrow_mut().right = Some(right.clone());
                    queue.push_back(right);
                }
                i += 1;
            }
        }
        Some(root)
    }

    // Example 1: [10,5,-3,3,2,null,11,3,-2,null,1] target=8 -> 3
    #[test]
    fn test_example1() {
        let tree = build_tree(&[
            Some(10),
            Some(5),
            Some(-3),
            Some(3),
            Some(2),
            None,
            Some(11),
            Some(3),
            Some(-2),
            None,
            Some(1),
        ]);
        assert_eq!(Solution::path_sum(tree.clone(), 8), 3);
        assert_eq!(Solution::path_sum_brute(tree, 8), 3);
    }

    // Example 2: [5,4,8,11,null,13,4,7,2,null,null,5,1] target=22 -> 3
    #[test]
    fn test_example2() {
        let tree = build_tree(&[
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            Some(5),
            Some(1),
        ]);
        assert_eq!(Solution::path_sum(tree.clone(), 22), 3);
        assert_eq!(Solution::path_sum_brute(tree, 22), 3);
    }

    // Empty tree -> 0
    #[test]
    fn test_empty() {
        assert_eq!(Solution::path_sum(None, 1), 0);
        assert_eq!(Solution::path_sum_brute(None, 1), 0);
    }

    // Single node match
    #[test]
    fn test_single_match() {
        let tree = build_tree(&[Some(5)]);
        assert_eq!(Solution::path_sum(tree.clone(), 5), 1);
        assert_eq!(Solution::path_sum_brute(tree, 5), 1);
    }

    // Single node no match
    #[test]
    fn test_single_no_match() {
        let tree = build_tree(&[Some(5)]);
        assert_eq!(Solution::path_sum(tree.clone(), 3), 0);
        assert_eq!(Solution::path_sum_brute(tree, 3), 0);
    }

    // Negative values: [-2, -3] target=-5 -> 1 (path -2 -> -3)
    #[test]
    fn test_negative() {
        let tree = build_tree(&[Some(-2), Some(-3)]);
        assert_eq!(Solution::path_sum(tree.clone(), -5), 1);
        assert_eq!(Solution::path_sum_brute(tree, -5), 1);
    }

    // Multiple paths: 1->1->1, target=2 -> 2 (top two nodes, bottom two nodes)
    #[test]
    fn test_multiple_paths() {
        let tree = build_tree(&[Some(1), Some(1), None, Some(1)]);
        assert_eq!(Solution::path_sum(tree.clone(), 2), 2);
        assert_eq!(Solution::path_sum_brute(tree, 2), 2);
    }

    // Zero target with all zeros: 0->0->0, target=0 -> 6
    // Paths: [root], [root->left], [root->left->left], [left], [left->left], [left-left]
    #[test]
    fn test_zero_target_all_zeros() {
        let tree = build_tree(&[Some(0), Some(0), None, Some(0)]);
        assert_eq!(Solution::path_sum(tree.clone(), 0), 6);
        assert_eq!(Solution::path_sum_brute(tree, 0), 6);
    }
}
