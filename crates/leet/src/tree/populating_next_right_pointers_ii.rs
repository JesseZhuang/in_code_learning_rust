//! LeetCode 117. Populating Next Right Pointers in Each Node II.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub type Link = Option<Rc<RefCell<Node>>>;

pub struct Node {
    pub val: i32,
    pub left: Link,
    pub right: Link,
    pub next: Link,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
            next: None,
        }
    }
}

pub struct Solution;

impl Solution {
    /// Connects nodes with breadth-first search in O(n) time and O(width) auxiliary space.
    pub fn connect(root: Link) -> Link {
        let mut queue: VecDeque<_> = root.iter().cloned().collect();

        while !queue.is_empty() {
            let level_size = queue.len();
            for index in 0..level_size {
                let node = queue.pop_front().unwrap();
                let mut node = node.borrow_mut();
                node.next = if index + 1 < level_size {
                    queue.front().cloned()
                } else {
                    None
                };
                queue.extend(
                    [node.left.clone(), node.right.clone()]
                        .into_iter()
                        .flatten(),
                );
            }
        }

        root
    }

    /// Uses each level's next links to build the following level in O(n) time and O(1) auxiliary space.
    pub fn connect_constant_space(root: Link) -> Link {
        let mut level_start = root.clone();
        if let Some(node) = &level_start {
            node.borrow_mut().next = None;
        }

        while level_start.is_some() {
            let mut current = level_start.take();
            let mut next_tail: Link = None;

            while let Some(parent) = current {
                let parent = parent.borrow();
                for child in [&parent.left, &parent.right].into_iter().flatten() {
                    if let Some(tail) = &next_tail {
                        tail.borrow_mut().next = Some(child.clone());
                    } else {
                        level_start = Some(child.clone());
                    }
                    next_tail = Some(child.clone());
                }
                current = parent.next.clone();
            }

            if let Some(tail) = next_tail {
                tail.borrow_mut().next = None;
            }
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONNECTORS: [fn(Link) -> Link; 2] = [Solution::connect, Solution::connect_constant_space];

    fn node(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node::new(val)))
    }

    fn assert_next(node: &Rc<RefCell<Node>>, expected: &Rc<RefCell<Node>>) {
        let node = node.borrow();
        let next = node.next.as_ref().expect("expected a next node");
        assert!(Rc::ptr_eq(next, expected));
    }

    #[test]
    fn empty_tree_returns_none() {
        for connect in CONNECTORS {
            assert!(connect(None).is_none());
        }
    }

    #[test]
    fn single_node_is_returned_with_no_next() {
        for connect in CONNECTORS {
            let root = node(1);

            let result = connect(Some(root.clone())).unwrap();

            assert!(Rc::ptr_eq(&result, &root));
            assert!(root.borrow().next.is_none());
        }
    }

    #[test]
    fn sparse_tree_links_across_parents() {
        for connect in CONNECTORS {
            let root = node(1);
            let left = node(2);
            let right = node(3);
            let left_left = node(4);
            let left_right = node(5);
            let right_right = node(6);
            let first_leaf = node(7);
            let second_leaf = node(7);
            root.borrow_mut().left = Some(left.clone());
            root.borrow_mut().right = Some(right.clone());
            left.borrow_mut().left = Some(left_left.clone());
            left.borrow_mut().right = Some(left_right.clone());
            right.borrow_mut().right = Some(right_right.clone());
            left_left.borrow_mut().right = Some(first_leaf.clone());
            right_right.borrow_mut().left = Some(second_leaf.clone());

            let result = connect(Some(root.clone())).unwrap();

            assert!(Rc::ptr_eq(&result, &root));
            assert_next(&left, &right);
            assert_next(&left_left, &left_right);
            assert_next(&left_right, &right_right);
            assert_next(&first_leaf, &second_leaf);
            for rightmost in [root, right, right_right, second_leaf] {
                assert!(rightmost.borrow().next.is_none());
            }
        }
    }

    #[test]
    fn one_sided_tree_ends_each_level() {
        for connect in CONNECTORS {
            let root = node(1);
            let child = node(2);
            let leaf = node(3);
            root.borrow_mut().right = Some(child.clone());
            child.borrow_mut().right = Some(leaf.clone());

            let result = connect(Some(root.clone())).unwrap();

            assert!(Rc::ptr_eq(&result, &root));
            for rightmost in [root, child, leaf] {
                assert!(rightmost.borrow().next.is_none());
            }
        }
    }
}
