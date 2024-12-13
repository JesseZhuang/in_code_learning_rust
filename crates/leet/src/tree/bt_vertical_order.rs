/// leet 314, lint 651

use crate::structs::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;


impl Solution {
    /// 20 ms, 3.16 mb
    pub fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        if root.is_none() { return res; }
        let mut col_v = HashMap::new();
        let (mut min_c, mut max_c, mut q) = (0, 0, VecDeque::new());
        q.push_back((root, 0));
        // must be in while let, while !q.is_empty(): None not covered
        while let Some((Some(n), c)) = q.pop_front() {
            let n = n.borrow();
            col_v.entry(c).or_insert(Vec::new()).push(n.val);
            min_c = min(min_c, c);
            max_c = max(max_c, c);
            if !n.left.is_none() { q.push_back((n.left.clone(), c - 1)) }
            if !n.right.is_none() { q.push_back((n.right.clone(), c + 1)) }
        }
        // move out of the map, alternatively can copy with .clone()
        for i in min_c..max_c + 1 { res.push(col_v.remove(&i).unwrap()) }
        res
    }
}

struct Solution;
