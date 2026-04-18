use std::cell::RefCell;
/// lc 230
use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut current = root;
        let mut count = 0;

        loop {
            while let Some(node) = current {
                stack.push(node.clone());
                current = node.borrow().left.clone();
            }
            let node = stack.pop().unwrap();
            count += 1;
            if count == k {
                return node.borrow().val;
            }
            current = node.borrow().right.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn test_example1() {
        // [3,1,4,null,2] k=1 -> 1
        let tree = node(3,
            node(1, None, node(2, None, None)),
            node(4, None, None),
        );
        assert_eq!(Solution::kth_smallest(tree, 1), 1);
    }

    #[test]
    fn test_example2() {
        // [5,3,6,2,4,null,null,1] k=3 -> 3
        let tree = node(5,
            node(3,
                node(2, node(1, None, None), None),
                node(4, None, None),
            ),
            node(6, None, None),
        );
        assert_eq!(Solution::kth_smallest(tree, 3), 3);
    }

    #[test]
    fn test_single_node() {
        let tree = node(42, None, None);
        assert_eq!(Solution::kth_smallest(tree, 1), 42);
    }
}
