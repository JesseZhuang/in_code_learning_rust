//! LeetCode 143 - Reorder List
//!
//! Given a singly linked list L: L0 → L1 → … → Ln-1 → Ln,
//! reorder it to: L0 → Ln → L1 → Ln-1 → L2 → Ln-2 → …
//!
//! Two approaches:
//! 1. Find middle, reverse second half, merge alternately. O(n) time, O(1) space.
//! 2. Collect into Vec, weave from both ends. O(n) time, O(n) space.

use crate::list::list_node::ListNode;

pub struct Solution;

impl Solution {
    /// Approach 1: Split at middle, reverse second half, merge.
    /// O(n) time, O(1) space.
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // Count length
        let mut len = 0;
        let mut cur = head.as_ref();
        while let Some(node) = cur {
            len += 1;
            cur = node.next.as_ref();
        }
        if len <= 2 {
            return;
        }

        // Split after the first half: first half has ceil(len/2) nodes
        let mid = (len + 1) / 2;
        let mut cur = head.as_mut().unwrap().as_mut();
        for _ in 1..mid {
            cur = cur.next.as_mut().unwrap().as_mut();
        }
        let mut second = cur.next.take();

        // Reverse second half
        let mut prev: Option<Box<ListNode>> = None;
        while let Some(mut node) = second {
            second = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        let mut second = prev;

        // Merge: interleave first and reversed second
        let mut cur = head.as_mut().unwrap().as_mut();
        while let Some(mut s_node) = second {
            second = s_node.next.take();
            s_node.next = cur.next.take();
            cur.next = Some(s_node);
            // Advance two steps (cur -> inserted -> next_original)
            let next = cur.next.as_mut().unwrap();
            if next.next.is_none() {
                break;
            }
            cur = next.next.as_mut().unwrap().as_mut();
        }
    }

    /// Approach 2: Vec-based. O(n) time, O(n) space.
    pub fn reorder_list_vec(head: &mut Option<Box<ListNode>>) {
        let mut nodes: Vec<Box<ListNode>> = Vec::new();
        let mut cur = head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
            nodes.push(node);
        }
        let n = nodes.len();
        if n <= 2 {
            // Rebuild and return
            let mut rebuilt: Option<Box<ListNode>> = None;
            for mut node in nodes.into_iter().rev() {
                node.next = rebuilt;
                rebuilt = Some(node);
            }
            *head = rebuilt;
            return;
        }

        // Weave: indices 0, n-1, 1, n-2, ...
        let mut order: Vec<usize> = Vec::with_capacity(n);
        let (mut lo, mut hi) = (0, n - 1);
        while lo <= hi {
            order.push(lo);
            if lo != hi {
                order.push(hi);
            }
            lo += 1;
            if hi == 0 {
                break;
            }
            hi -= 1;
        }

        // Build list in reverse order of `order`
        let mut result: Option<Box<ListNode>> = None;
        for &idx in order.iter().rev() {
            let mut node = std::mem::replace(&mut nodes[idx], Box::new(ListNode::new(0)));
            node.next = result;
            result = Some(node);
        }
        *head = result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list::list_node::{from_vec, to_vec};

    fn run_reorder(input: &[i32], expected: &[i32]) {
        // Test approach 1
        let mut list = from_vec(input);
        Solution::reorder_list(&mut list);
        assert_eq!(to_vec(list), expected);

        // Test approach 2
        let mut list = from_vec(input);
        Solution::reorder_list_vec(&mut list);
        assert_eq!(to_vec(list), expected);
    }

    #[test]
    fn test_four_elements() {
        run_reorder(&[1, 2, 3, 4], &[1, 4, 2, 3]);
    }

    #[test]
    fn test_five_elements() {
        run_reorder(&[1, 2, 3, 4, 5], &[1, 5, 2, 4, 3]);
    }

    #[test]
    fn test_single() {
        run_reorder(&[1], &[1]);
    }

    #[test]
    fn test_two() {
        run_reorder(&[1, 2], &[1, 2]);
    }

    #[test]
    fn test_three() {
        run_reorder(&[1, 2, 3], &[1, 3, 2]);
    }

    #[test]
    fn test_six_elements() {
        run_reorder(&[1, 2, 3, 4, 5, 6], &[1, 6, 2, 5, 3, 4]);
    }
}
