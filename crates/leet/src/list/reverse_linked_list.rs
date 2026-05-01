//! LeetCode 206 / LintCode 35, easy, tags: linked list, recursion.

use crate::list::list_node::ListNode;

pub struct Solution;

impl Solution {
    /// Iterative. Time O(n), Space O(1).
    pub fn reverse_list_iterative(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res: Option<Box<ListNode>> = None;
        let mut cur = head;
        while let Some(mut node) = cur {
            cur = node.next.take();   // detach successor before re-pointing next
            node.next = res;          // reverse: point at previously-built reversed prefix
            res = Some(node);
        }
        res
    }

    /// Recursive. Time O(n), Space O(n) recursion stack.
    pub fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(mut node) => match node.next.take() {
                None => Some(node),                  // single node, already reversed
                Some(next) => {
                    let mut new_head = Self::reverse_list_recursive(Some(next))
                        .expect("non-empty after recursion");
                    // Walk to the tail of the reversed remainder and append `node` there.
                    // (Same shape as the Java/Python `head.next.next = head` trick;
                    // ownership rules force us to traverse instead of using a back pointer.)
                    let mut tail_walker = &mut new_head;
                    while tail_walker.next.is_some() {
                        tail_walker = tail_walker.next.as_mut().unwrap();
                    }
                    tail_walker.next = Some(node);
                    Some(new_head)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list::list_node::{from_vec, to_vec};

    fn verify(input: &[i32], expected: &[i32]) {
        assert_eq!(
            to_vec(Solution::reverse_list_iterative(from_vec(input))),
            expected.to_vec(),
            "iterative {:?}",
            input
        );
        assert_eq!(
            to_vec(Solution::reverse_list_recursive(from_vec(input))),
            expected.to_vec(),
            "recursive {:?}",
            input
        );
    }

    #[test]
    fn example1() {
        verify(&[1, 2, 3, 4, 5], &[5, 4, 3, 2, 1]);
    }

    #[test]
    fn two_nodes() {
        verify(&[1, 2], &[2, 1]);
    }

    #[test]
    fn empty() {
        verify(&[], &[]);
    }

    #[test]
    fn single_node() {
        verify(&[42], &[42]);
    }

    #[test]
    fn negative_values() {
        verify(&[-5, 0, 5, -10], &[-10, 5, 0, -5]);
    }

    #[test]
    fn duplicates() {
        verify(&[1, 1, 2, 2, 3, 3], &[3, 3, 2, 2, 1, 1]);
    }

    #[test]
    fn long_list() {
        let input: Vec<i32> = (1..=100).collect();
        let expected: Vec<i32> = (1..=100).rev().collect();
        verify(&input, &expected);
    }
}
