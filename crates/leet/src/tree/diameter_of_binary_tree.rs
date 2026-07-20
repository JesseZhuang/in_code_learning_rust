use std::cell::RefCell;
use std::cmp::max;
/// lc 543

use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn depth(node: Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
            match node {
                None => 0, // base case
                Some(n) => {
                    let n = n.borrow();
                    let left = depth(n.left.clone(), diameter); // O(h) stack
                    let right = depth(n.right.clone(), diameter);
                    *diameter = max(*diameter, left + right); // diameter = left_depth + right_depth
                    max(left, right) + 1 // return depth to parent
                }
            }
        }
        let mut diameter = 0;
        depth(root, &mut diameter); // O(n) time, O(h) space
        diameter
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

    #[test]
    fn test_example1() {
        // [1,2,3,4,5] -> diameter 3 (path 4-2-1-3 or 5-2-1-3)
        assert_eq!(Solution::diameter_of_binary_tree(tree(&[1, 2, 3, 4, 5])), 3);
    }

    #[test]
    fn test_example2() {
        // [1,2] -> diameter 1
        assert_eq!(Solution::diameter_of_binary_tree(tree(&[1, 2])), 1);
    }

    #[test]
    fn test_single_node() {
        assert_eq!(Solution::diameter_of_binary_tree(tree(&[1])), 0);
    }

    #[test]
    fn test_none() {
        assert_eq!(Solution::diameter_of_binary_tree(None), 0);
    }

    #[test]
    fn test_linear_left() {
        // 1-2-3-4 (left skewed, depth 3 edges)
        let n = i32::MIN;
        assert_eq!(Solution::diameter_of_binary_tree(tree(&[1, 2, n, 3, n, 4])), 3);
    }

    #[test]
    fn test_diameter_not_through_root() {
        // Tree where longest path doesn't go through root:
        //        1
        //       /
        //      2
        //     / \
        //    3   4
        //   /     \
        //  5       6
        // Diameter: 5-3-2-4-6 = 4 edges
        let n = i32::MIN;
        assert_eq!(
            Solution::diameter_of_binary_tree(tree(&[1, 2, n, 3, 4, 5, n, n, 6])),
            4
        );
    }

    #[test]
    fn test_balanced() {
        // Perfect binary tree depth 3:
        //         1
        //       /   \
        //      2     3
        //     / \   / \
        //    4   5 6   7
        // Diameter: 4 (e.g. 4-2-1-3-7)
        assert_eq!(Solution::diameter_of_binary_tree(tree(&[1, 2, 3, 4, 5, 6, 7])), 4);
    }
}
