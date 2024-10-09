use std::cell::RefCell;
use std::cmp::max;
/// lc 104

use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(n) => {
                1 + max(Self::max_depth(n.borrow().left.clone()),
                        Self::max_depth(n.borrow().right.clone()))
            }
            None => 0
        }
    }
}

pub struct Solution;