use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

pub struct Solution;

impl Solution {
    /// BFS approach: build adjacency list via DFS, then BFS from start.
    /// O(n) time, O(n) space.
    pub fn amount_of_time(root: TreeLink, start: i32) -> i32 {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

        // Build adjacency list
        fn build(node: &TreeLink, graph: &mut HashMap<i32, Vec<i32>>) {
            if let Some(n) = node {
                let n = n.borrow();
                if let Some(ref left) = n.left {
                    let lv = left.borrow().val;
                    graph.entry(n.val).or_default().push(lv);
                    graph.entry(lv).or_default().push(n.val);
                    build(&n.left, graph);
                }
                if let Some(ref right) = n.right {
                    let rv = right.borrow().val;
                    graph.entry(n.val).or_default().push(rv);
                    graph.entry(rv).or_default().push(n.val);
                    build(&n.right, graph);
                }
            }
        }

        build(&root, &mut graph);

        // BFS from start
        let mut visited: HashMap<i32, bool> = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);
        visited.insert(start, true);
        let mut minutes = -1;

        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let cur = queue.pop_front().unwrap();
                if let Some(neighbors) = graph.get(&cur) {
                    for &nb in neighbors {
                        if !visited.contains_key(&nb) {
                            visited.insert(nb, true);
                            queue.push_back(nb);
                        }
                    }
                }
            }
            minutes += 1;
        }

        minutes
    }

    /// Pure DFS approach: encode distance as negative.
    /// O(n) time, O(h) space.
    pub fn amount_of_time_dfs(root: TreeLink, start: i32) -> i32 {
        let mut ans = 0;

        fn depth(node: &TreeLink, start: i32, ans: &mut i32) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    let left = depth(&n.left, start, ans);
                    let right = depth(&n.right, start, ans);

                    if n.val == start {
                        // Downward distance is max of children heights
                        *ans = left.max(right);
                        return -1;
                    }

                    if left < 0 {
                        // start is in left subtree
                        // distance = right height + distance from start to current
                        *ans = (*ans).max(right - left);
                        return left - 1;
                    }

                    if right < 0 {
                        // start is in right subtree
                        *ans = (*ans).max(left - right);
                        return right - 1;
                    }

                    // Normal case: return height
                    left.max(right) + 1
                }
            }
        }

        depth(&root, start, &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(val: i32, left: TreeLink, right: TreeLink) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    fn leaf(val: i32) -> TreeLink {
        node(val, None, None)
    }

    // tree (1(5(4(9,2),nil),3(10,6))), start=3, expected=4
    #[test]
    fn test_example1() {
        let tree = node(
            1,
            node(5, node(4, leaf(9), leaf(2)), None),
            node(3, leaf(10), leaf(6)),
        );
        assert_eq!(Solution::amount_of_time(tree.clone(), 3), 4);
        assert_eq!(Solution::amount_of_time_dfs(tree, 3), 4);
    }

    // tree(1), start=1, expected=0
    #[test]
    fn test_single() {
        let tree = leaf(1);
        assert_eq!(Solution::amount_of_time(tree.clone(), 1), 0);
        assert_eq!(Solution::amount_of_time_dfs(tree, 1), 0);
    }

    // tree(1(2(4,nil),3)), start=1, expected=2
    #[test]
    fn test_start_root() {
        let tree = node(1, node(2, leaf(4), None), leaf(3));
        assert_eq!(Solution::amount_of_time(tree.clone(), 1), 2);
        assert_eq!(Solution::amount_of_time_dfs(tree, 1), 2);
    }

    // tree(1(2(4,5),3)), start=4, expected=3
    #[test]
    fn test_start_leaf() {
        let tree = node(1, node(2, leaf(4), leaf(5)), leaf(3));
        assert_eq!(Solution::amount_of_time(tree.clone(), 4), 3);
        assert_eq!(Solution::amount_of_time_dfs(tree, 4), 3);
    }

    // tree(1(2(3(4,nil),nil),nil)), start=2, expected=2
    #[test]
    fn test_linear() {
        let tree = node(1, node(2, node(3, leaf(4), None), None), None);
        assert_eq!(Solution::amount_of_time(tree.clone(), 2), 2);
        assert_eq!(Solution::amount_of_time_dfs(tree, 2), 2);
    }

    // tree(1(2,3(nil,4(nil,5)))), start=1, expected=3
    #[test]
    fn test_deep_right() {
        let tree = node(1, leaf(2), node(3, None, node(4, None, leaf(5))));
        assert_eq!(Solution::amount_of_time(tree.clone(), 1), 3);
        assert_eq!(Solution::amount_of_time_dfs(tree, 1), 3);
    }
}
