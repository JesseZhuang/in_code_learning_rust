use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    // --- DFS (preorder) ---

    pub fn serialize_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = Vec::new();
        Self::serialize_dfs_helper(&root, &mut result);
        result.join(",")
    }

    fn serialize_dfs_helper(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<String>) {
        match node {
            None => result.push("null".to_string()),
            Some(n) => {
                let n = n.borrow();
                result.push(n.val.to_string());
                Self::serialize_dfs_helper(&n.left, result);
                Self::serialize_dfs_helper(&n.right, result);
            }
        }
    }

    pub fn deserialize_dfs(data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
        let tokens: Vec<&str> = data.split(',').collect();
        let mut idx = 0;
        Self::deserialize_dfs_helper(&tokens, &mut idx)
    }

    fn deserialize_dfs_helper(tokens: &[&str], idx: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        if *idx >= tokens.len() {
            return None;
        }
        let token = tokens[*idx];
        *idx += 1;
        if token == "null" {
            return None;
        }
        let val: i32 = token.parse().unwrap();
        let left = Self::deserialize_dfs_helper(tokens, idx);
        let right = Self::deserialize_dfs_helper(tokens, idx);
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    // --- BFS (level-order) ---

    pub fn serialize_bfs(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while let Some(node) = queue.pop_front() {
            match node {
                None => result.push("null".to_string()),
                Some(n) => {
                    let n = n.borrow();
                    result.push(n.val.to_string());
                    queue.push_back(n.left.clone());
                    queue.push_back(n.right.clone());
                }
            }
        }
        // Trim trailing nulls for cleaner output
        while result.last().map_or(false, |s| s == "null") {
            result.pop();
        }
        result.join(",")
    }

    pub fn deserialize_bfs(data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
        let tokens: Vec<&str> = data.split(',').collect();
        if tokens.is_empty() || tokens[0] == "null" {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(tokens[0].parse().unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));
        let mut i = 1;
        while let Some(node) = queue.pop_front() {
            // left child
            if i < tokens.len() {
                if tokens[i] != "null" {
                    let left = Rc::new(RefCell::new(TreeNode::new(tokens[i].parse().unwrap())));
                    node.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }
            // right child
            if i < tokens.len() {
                if tokens[i] != "null" {
                    let right = Rc::new(RefCell::new(TreeNode::new(tokens[i].parse().unwrap())));
                    node.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }
        Some(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a tree from a level-order slice (None = null).
    fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));
        let mut i = 1;
        while let Some(node) = queue.pop_front() {
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }
        Some(root)
    }

    /// Compare two trees for structural and value equality.
    fn trees_equal(
        a: &Option<Rc<RefCell<TreeNode>>>,
        b: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                let a = a.borrow();
                let b = b.borrow();
                a.val == b.val
                    && trees_equal(&a.left, &b.left)
                    && trees_equal(&a.right, &b.right)
            }
            _ => false,
        }
    }

    fn roundtrip_dfs(vals: &[Option<i32>]) {
        let tree = build_tree(vals);
        let serialized = Solution::serialize_dfs(tree.clone());
        let deserialized = Solution::deserialize_dfs(serialized);
        assert!(trees_equal(&tree, &deserialized));
    }

    fn roundtrip_bfs(vals: &[Option<i32>]) {
        let tree = build_tree(vals);
        let serialized = Solution::serialize_bfs(tree.clone());
        let deserialized = Solution::deserialize_bfs(serialized);
        assert!(trees_equal(&tree, &deserialized));
    }

    #[test]
    fn test_example_tree() {
        // [1,2,3,null,null,4,5]
        let vals = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5)];
        roundtrip_dfs(&vals);
        roundtrip_bfs(&vals);
    }

    #[test]
    fn test_empty_tree() {
        roundtrip_dfs(&[]);
        roundtrip_bfs(&[]);
    }

    #[test]
    fn test_single_node() {
        roundtrip_dfs(&[Some(42)]);
        roundtrip_bfs(&[Some(42)]);
    }

    #[test]
    fn test_left_skewed() {
        // 1 -> 2 -> 3 -> 4 (all left children)
        let vals = vec![Some(1), Some(2), None, Some(3), None, Some(4)];
        roundtrip_dfs(&vals);
        roundtrip_bfs(&vals);
    }

    #[test]
    fn test_right_skewed() {
        // 1 -> 2 -> 3 -> 4 (all right children)
        let vals = vec![Some(1), None, Some(2), None, Some(3), None, Some(4)];
        roundtrip_dfs(&vals);
        roundtrip_bfs(&vals);
    }

    #[test]
    fn test_negative_values() {
        let vals = vec![Some(-1), Some(-2), Some(-3), None, None, Some(-4), Some(5)];
        roundtrip_dfs(&vals);
        roundtrip_bfs(&vals);
    }
}
