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
        // [1,2,3] -> 6
        assert_eq!(Solution::max_path_sum(tree(&[1, 2, 3])), 6);
    }

    #[test]
    fn test_example2() {
        // [-10,9,20,null,null,15,7] -> 42
        let n = i32::MIN;
        assert_eq!(Solution::max_path_sum(tree(&[-10, 9, 20, n, n, 15, 7])), 42);
    }

    #[test]
    fn test_single_node() {
        assert_eq!(Solution::max_path_sum(tree(&[1])), 1);
    }

    #[test]
    fn test_single_negative() {
        assert_eq!(Solution::max_path_sum(tree(&[-3])), -3);
    }

    #[test]
    fn test_all_negative() {
        // [-1,-2,-3] -> -1
        assert_eq!(Solution::max_path_sum(tree(&[-1, -2, -3])), -1);
    }

    #[test]
    fn test_path_not_through_root() {
        // [1,2,null,3,4] -> 1+2+3+4 not valid; path is 3-2-4=9, plus root: 1+2+4=7? no.
        // Tree:   1
        //        /
        //       2
        //      / \
        //     3   4
        // Best path: 3-2-4 = 9, or 3-2-1 = 6, or 4-2-1 = 7. Wait: 3+2+4=9, 1+2+4=7, 1+2+3=6
        // But also 1+2+3=6... max is 3+2+4 = 9? No wait let me recalc.
        // maxGain(3)=3, maxGain(4)=4, at node 2: res=max(res,3+4+2)=9, return max(3,4)+2=6
        // at node 1: left=6, right=0, res=max(9,6+0+1)=9, return 6+1=7
        // So answer is 10? Let me recheck. Tree [1,2,i32::MIN,3,4]:
        // root=1, left=2, right=null. Node 2: left=3, right=4.
        // maxGain(3)=3, maxGain(4)=4. At 2: res=3+4+2=9, return 4+2=6.
        // At 1: left=6, right=0. res=max(9, 6+0+1)=9. Hmm, not 10.
        // Actually path 1-2-4 = 7, path 3-2-4 = 9, path 1-2-3 = 6. Answer = 9.
        let n = i32::MIN;
        assert_eq!(Solution::max_path_sum(tree(&[1, 2, n, 3, 4])), 9);
    }

    #[test]
    fn test_left_skewed() {
        // [-2, 1] -> 1
        let n = i32::MIN;
        assert_eq!(Solution::max_path_sum(tree(&[-2, 1, n])), 1);
    }

    #[test]
    fn test_boundary_values() {
        // [1000, -1000, 1000] -> 2000
        assert_eq!(Solution::max_path_sum(tree(&[1000, -1000, 1000])), 2000);
    }
}