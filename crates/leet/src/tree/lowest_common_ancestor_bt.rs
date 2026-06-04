/// LeetCode 236: Lowest Common Ancestor of a Binary Tree

use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;

/// Solution 1: Recursive DFS
/// Time: O(n), Space: O(h)
pub struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(root: Node, p: Node, q: Node) -> Node {
        let p_val = p.as_ref()?.borrow().val;
        let q_val = q.as_ref()?.borrow().val;
        Self::dfs(&root, p_val, q_val)
    }

    fn dfs(node: &Node, p: i32, q: i32) -> Node {
        let n = node.as_ref()?;
        let val = n.borrow().val;
        if val == p || val == q {
            return node.clone();
        }
        let left = Self::dfs(&n.borrow().left, p, q);
        let right = Self::dfs(&n.borrow().right, p, q);
        match (left.is_some(), right.is_some()) {
            (true, true) => node.clone(),
            (true, false) => left,
            (false, true) => right,
            _ => None,
        }
    }
}

/// Solution 2: Iterative with parent HashMap
/// Time: O(n), Space: O(n)
pub struct Solution2;

impl Solution2 {
    pub fn lowest_common_ancestor(root: Node, p: Node, q: Node) -> Node {
        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;

        // BFS to build parent map
        let mut parent: HashMap<i32, Node> = HashMap::new();
        parent.insert(root.as_ref().unwrap().borrow().val, None);

        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        while !parent.contains_key(&p_val) || !parent.contains_key(&q_val) {
            let node = queue.pop_front().unwrap();
            let n = node.as_ref().unwrap().borrow();
            if let Some(ref left) = n.left {
                parent.insert(left.borrow().val, node.clone());
                queue.push_back(Some(left.clone()));
            }
            if let Some(ref right) = n.right {
                parent.insert(right.borrow().val, node.clone());
                queue.push_back(Some(right.clone()));
            }
        }

        // Trace ancestors of p
        let mut ancestors: HashSet<i32> = HashSet::new();
        let mut cur_val = Some(p_val);
        while let Some(v) = cur_val {
            ancestors.insert(v);
            cur_val = parent.get(&v).and_then(|p| p.as_ref().map(|n| n.borrow().val));
        }

        // Walk up from q until we hit an ancestor of p
        let mut cur_val = Some(q_val);
        while let Some(v) = cur_val {
            if ancestors.contains(&v) {
                // Find and return the node with this value
                return Self::find_node(&root, v);
            }
            cur_val = parent.get(&v).and_then(|p| p.as_ref().map(|n| n.borrow().val));
        }
        None
    }

    fn find_node(root: &Node, val: i32) -> Node {
        let n = root.as_ref()?;
        if n.borrow().val == val {
            return root.clone();
        }
        Self::find_node(&n.borrow().left, val)
            .or_else(|| Self::find_node(&n.borrow().right, val))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build tree from level-order array (None = null)
    fn build_tree(vals: &[Option<i32>]) -> Node {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode {
            val: vals[0].unwrap(),
            left: None,
            right: None,
        }));
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;
        while i < vals.len() {
            let node = queue.pop_front().unwrap();
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode { val: v, left: None, right: None }));
                    node.borrow_mut().left = Some(left.clone());
                    queue.push_back(left);
                }
                i += 1;
            }
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode { val: v, left: None, right: None }));
                    node.borrow_mut().right = Some(right.clone());
                    queue.push_back(right);
                }
                i += 1;
            }
        }
        Some(root)
    }

    fn find_node(root: &Node, val: i32) -> Node {
        let n = root.as_ref()?;
        if n.borrow().val == val {
            return root.clone();
        }
        find_node(&n.borrow().left, val).or_else(|| find_node(&n.borrow().right, val))
    }

    fn get_val(node: &Node) -> i32 {
        node.as_ref().unwrap().borrow().val
    }

    //  Tree: [3,5,1,6,2,0,8,null,null,7,4]
    fn big_tree() -> Node {
        build_tree(&[
            Some(3), Some(5), Some(1), Some(6), Some(2),
            Some(0), Some(8), None, None, Some(7), Some(4),
        ])
    }

    #[test]
    fn test_solution1_p5_q1() {
        let root = big_tree();
        let p = find_node(&root, 5);
        let q = find_node(&root, 1);
        let res = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(get_val(&res), 3);
    }

    #[test]
    fn test_solution1_p5_q4() {
        let root = big_tree();
        let p = find_node(&root, 5);
        let q = find_node(&root, 4);
        let res = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(get_val(&res), 5);
    }

    #[test]
    fn test_solution1_tree12() {
        let root = build_tree(&[Some(1), Some(2)]);
        let p = find_node(&root, 1);
        let q = find_node(&root, 2);
        let res = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(get_val(&res), 1);
    }

    #[test]
    fn test_solution1_p7_q4() {
        let root = big_tree();
        let p = find_node(&root, 7);
        let q = find_node(&root, 4);
        let res = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(get_val(&res), 2);
    }

    #[test]
    fn test_solution2_p5_q1() {
        let root = big_tree();
        let p = find_node(&root, 5);
        let q = find_node(&root, 1);
        let res = Solution2::lowest_common_ancestor(root, p, q);
        assert_eq!(get_val(&res), 3);
    }

    #[test]
    fn test_solution2_p5_q4() {
        let root = big_tree();
        let p = find_node(&root, 5);
        let q = find_node(&root, 4);
        let res = Solution2::lowest_common_ancestor(root, p, q);
        assert_eq!(get_val(&res), 5);
    }

    #[test]
    fn test_solution2_tree12() {
        let root = build_tree(&[Some(1), Some(2)]);
        let p = find_node(&root, 1);
        let q = find_node(&root, 2);
        let res = Solution2::lowest_common_ancestor(root, p, q);
        assert_eq!(get_val(&res), 1);
    }

    #[test]
    fn test_solution2_p7_q4() {
        let root = big_tree();
        let p = find_node(&root, 7);
        let q = find_node(&root, 4);
        let res = Solution2::lowest_common_ancestor(root, p, q);
        assert_eq!(get_val(&res), 2);
    }
}
