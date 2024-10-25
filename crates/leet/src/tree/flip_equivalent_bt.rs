// lc 951

use crate::structs::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn flip_equiv(r1: Option<Rc<RefCell<TreeNode>>>,
                      r2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if r1.is_none() || r2.is_none() { return r1 == r2; }
        let (r1, r2) = (r1.unwrap(), r2.unwrap());
        let (r1, r2) = (r1.borrow(), r2.borrow());
        r1.val == r2.val && (
            Self::flip_equiv(r1.left.clone(), r2.left.clone()) &&
                Self::flip_equiv(r1.right.clone(), r2.right.clone()) ||
                Self::flip_equiv(r1.left.clone(), r2.right.clone()) &&
                    Self::flip_equiv(r1.right.clone(), r2.left.clone())
        )
    }
}

struct Solution;
