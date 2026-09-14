use std::cell::RefCell;
/// lc 572
use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

pub struct Solution;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    /// Recursive DFS: for each node in root, check if the subtree rooted there
    /// is identical to sub_root. Time O(m*n), Space O(m) stack.
    pub fn is_subtree(root: Node, sub_root: Node) -> bool {
        fn is_same(a: &Node, b: &Node) -> bool {
            match (a, b) {
                (None, None) => true,
                (Some(a), Some(b)) => {
                    let a = a.borrow();
                    let b = b.borrow();
                    a.val == b.val && is_same(&a.left, &b.left) && is_same(&a.right, &b.right)
                }
                _ => false,
            }
        }
        fn check(root: &Node, sub: &Node) -> bool {
            match root {
                None => false,
                Some(n) => {
                    if is_same(root, sub) {
                        return true;
                    }
                    let n = n.borrow();
                    check(&n.left, sub) || check(&n.right, sub)
                }
            }
        }
        check(&root, &sub_root)
    }

    /// Serialization: serialize both trees to strings using pre-order traversal
    /// with null markers and value delimiters, then check substring containment.
    /// Delimiters prevent false prefix matches (e.g. "12" vs "2").
    /// Time O(m+n), Space O(m+n).
    pub fn is_subtree_serial(root: Node, sub_root: Node) -> bool {
        fn serialize(node: &Node, buf: &mut String) {
            match node {
                None => buf.push_str(",#"),
                Some(n) => {
                    let n = n.borrow();
                    buf.push(',');
                    buf.push_str(&n.val.to_string());
                    serialize(&n.left, buf);
                    serialize(&n.right, buf);
                }
            }
        }
        let mut s_root = String::new();
        serialize(&root, &mut s_root);
        let mut s_sub = String::new();
        serialize(&sub_root, &mut s_sub);
        s_root.contains(&s_sub)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn build_tree(vals: &[Option<i32>]) -> Node {
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

    fn tree(vals: &[i32]) -> Node {
        let opts: Vec<Option<i32>> = vals
            .iter()
            .map(|&v| if v == i32::MIN { None } else { Some(v) })
            .collect();
        build_tree(&opts)
    }

    // Helper to run both solutions and assert same result
    fn check(root: &[i32], sub: &[i32], expected: bool) {
        assert_eq!(
            Solution::is_subtree(tree(root), tree(sub)),
            expected,
            "DFS failed for root={:?}, sub={:?}",
            root,
            sub
        );
        assert_eq!(
            Solution::is_subtree_serial(tree(root), tree(sub)),
            expected,
            "Serial failed for root={:?}, sub={:?}",
            root,
            sub
        );
    }

    #[test]
    fn test_example1() {
        // root=[3,4,5,1,2], sub=[4,1,2] → true
        check(&[3, 4, 5, 1, 2], &[4, 1, 2], true);
    }

    #[test]
    fn test_example2_subtree_mismatch() {
        // root=[3,4,5,1,2,N,N,N,N,0], sub=[4,1,2] → false
        // The subtree rooted at 4 has an extra child 0 under 2.
        let n = i32::MIN;
        check(&[3, 4, 5, 1, 2, n, n, n, n, 0], &[4, 1, 2], false);
    }

    #[test]
    fn test_single_node_match() {
        check(&[1], &[1], true);
    }

    #[test]
    fn test_single_node_no_match() {
        check(&[1], &[2], false);
    }

    #[test]
    fn test_value_prefix_trap() {
        // root=[12], sub=[2] → false (value 12 != 2)
        check(&[12], &[2], false);
    }

    #[test]
    fn test_negative_values() {
        check(&[-1, -2, -3], &[-2], true);
    }

    #[test]
    fn test_negative_no_match() {
        check(&[-1, -2, -3], &[-4], false);
    }

    #[test]
    fn test_deep_right_chain() {
        let n = i32::MIN;
        // root: 1 -> right 2 -> right 3 -> right 4
        // sub: 3 -> right 4
        check(&[1, n, 2, n, 3, n, 4], &[3, n, 4], true);
    }

    #[test]
    fn test_deep_right_chain_no_match() {
        let n = i32::MIN;
        // sub has left child instead of right
        check(&[1, n, 2, n, 3, n, 4], &[3, 4], false);
    }

    #[test]
    fn test_sub_at_leaf() {
        check(&[1, 2, 3], &[3], true);
    }
}
