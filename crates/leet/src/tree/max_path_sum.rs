use crate::structs::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
/// lc 124

use std::rc::Rc;

impl Solution {
    // credit LuKuangChen@
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn max_gain(env: &mut Env, node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    let left = max(0, max_gain(env, n.left.clone()));
                    let right = max(0, max_gain(env, n.right.clone()));
                    env.res = max(env.res, left + right + n.val);
                    max(left, right) + n.val
                }
            }
        }
        let env = &mut Env { res: i32::MIN };
        max_gain(env, root);
        env.res
    }
}

struct Env {
    res: i32,
}

struct Solution;