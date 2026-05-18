/// LeetCode 2583: Kth Largest Sum in a Binary Tree

use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    /// BFS + sort approach.
    /// O(n) time for BFS, O(d log d) for sorting level sums where d = depth.
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut level_sums: Vec<i64> = Vec::new();
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new(); // O(width) space

        if let Some(node) = root {
            queue.push_back(node);
        }

        while !queue.is_empty() {
            let size = queue.len(); // nodes at current level
            let mut level_sum: i64 = 0;
            for _ in 0..size {
                let node = queue.pop_front().unwrap();
                let n = node.borrow();
                level_sum += n.val as i64; // accumulate level sum
                if let Some(ref left) = n.left {
                    queue.push_back(Rc::clone(left));
                }
                if let Some(ref right) = n.right {
                    queue.push_back(Rc::clone(right));
                }
            }
            level_sums.push(level_sum);
        }

        if (k as usize) > level_sums.len() {
            return -1; // fewer than k levels
        }

        level_sums.sort_unstable_by(|a, b| b.cmp(a)); // O(d log d) descending sort
        level_sums[(k - 1) as usize]
    }

    /// BFS + min-heap approach.
    /// O(n) time for BFS, O(k) space for the heap (keeps only top-k sums).
    pub fn kth_largest_level_sum_heap(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let k = k as usize;
        let mut heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new(); // min-heap via Reverse, O(k) space
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

        if let Some(node) = root {
            queue.push_back(node);
        }

        while !queue.is_empty() {
            let size = queue.len();
            let mut level_sum: i64 = 0;
            for _ in 0..size {
                let node = queue.pop_front().unwrap();
                let n = node.borrow();
                level_sum += n.val as i64;
                if let Some(ref left) = n.left {
                    queue.push_back(Rc::clone(left));
                }
                if let Some(ref right) = n.right {
                    queue.push_back(Rc::clone(right));
                }
            }
            // Maintain a min-heap of size k (top-k largest)
            if heap.len() < k {
                heap.push(Reverse(level_sum)); // O(log k) push
            } else if let Some(&Reverse(min)) = heap.peek() {
                if level_sum > min {
                    heap.pop(); // O(log k) pop
                    heap.push(Reverse(level_sum)); // O(log k) push
                }
            }
        }

        if heap.len() < k {
            -1 // fewer than k levels
        } else {
            heap.peek().unwrap().0 // kth largest is the min in top-k heap
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const I32_NULL: i32 = i32::MIN; // sentinel for null nodes

    /// Build a binary tree from level-order array (LeetCode style, with I32_NULL for null).
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

    /// Helper: convert &[i32] with I32_NULL sentinel to Option slice
    fn to_opts(vals: &[i32]) -> Vec<Option<i32>> {
        vals.iter()
            .map(|&v| if v == I32_NULL { None } else { Some(v) })
            .collect()
    }

    // Example 1: [5,8,9,2,1,3,7,4], k=2 → 13
    // Level 0: 5, Level 1: 8+9=17, Level 2: 2+1+3+7=13, Level 3: 4
    #[test]
    fn test_example1() {
        // Tree: root=5, children 8,9, then 2,1,3,7, then 4,6
        // Level sums: 5, 17, 13, 10 → sorted desc: 17, 13, 10, 5 → k=2 → 13
        let tree = build_tree(&to_opts(&[5, 8, 9, 2, 1, 3, 7, 4, 6]));
        assert_eq!(Solution::kth_largest_level_sum(tree.clone(), 2), 13);
        let tree = build_tree(&to_opts(&[5, 8, 9, 2, 1, 3, 7, 4, 6]));
        assert_eq!(Solution::kth_largest_level_sum_heap(tree.clone(), 2), 13);
    }

    // Example 2: [1,2,null,3], k=1 → 3
    // Level 0: 1, Level 1: 2, Level 2: 3 → sorted desc: 3, 2, 1 → k=1 → 3
    #[test]
    fn test_example2() {
        let tree = build_tree(&to_opts(&[1, 2, I32_NULL, 3]));
        assert_eq!(Solution::kth_largest_level_sum(tree.clone(), 1), 3);
        let tree = build_tree(&to_opts(&[1, 2, I32_NULL, 3]));
        assert_eq!(Solution::kth_largest_level_sum_heap(tree.clone(), 1), 3);
    }

    // k exceeds depth → -1
    #[test]
    fn test_k_exceeds_depth() {
        let tree = build_tree(&to_opts(&[1, 2, 3]));
        assert_eq!(Solution::kth_largest_level_sum(tree.clone(), 5), -1);
        let tree = build_tree(&to_opts(&[1, 2, 3]));
        assert_eq!(Solution::kth_largest_level_sum_heap(tree.clone(), 5), -1);
    }

    // Single node
    #[test]
    fn test_single_node() {
        let tree = build_tree(&to_opts(&[42]));
        assert_eq!(Solution::kth_largest_level_sum(tree.clone(), 1), 42);
        let tree = build_tree(&to_opts(&[42]));
        assert_eq!(Solution::kth_largest_level_sum_heap(tree.clone(), 1), 42);
    }

    // Negative values
    #[test]
    fn test_negative_values() {
        // Tree: [-10, -5, -3, -1, -2]
        // Level 0: -10, Level 1: -5 + -3 = -8, Level 2: -1 + -2 = -3
        // Sorted desc: -3, -8, -10 → k=1 → -3, k=2 → -8, k=3 → -10
        let tree = build_tree(&to_opts(&[-10, -5, -3, -1, -2]));
        assert_eq!(Solution::kth_largest_level_sum(tree.clone(), 1), -3);
        let tree = build_tree(&to_opts(&[-10, -5, -3, -1, -2]));
        assert_eq!(Solution::kth_largest_level_sum(tree.clone(), 2), -8);
        let tree = build_tree(&to_opts(&[-10, -5, -3, -1, -2]));
        assert_eq!(Solution::kth_largest_level_sum_heap(tree.clone(), 1), -3);
        let tree = build_tree(&to_opts(&[-10, -5, -3, -1, -2]));
        assert_eq!(Solution::kth_largest_level_sum_heap(tree.clone(), 3), -10);
    }
}
