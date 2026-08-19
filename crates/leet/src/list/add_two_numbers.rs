//! LeetCode 2 — Add Two Numbers
//!
//! Two non-empty linked lists represent non-negative integers in **reverse** order.
//! Return their sum as a linked list, also in reverse order.

use crate::list::list_node::ListNode;

pub struct Solution;

impl Solution {
    /// Iterative approach with dummy head.
    /// Time: O(max(m, n)), Space: O(max(m, n)) for the result list.
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut p1 = l1;
        let mut p2 = l2;
        let mut carry = 0;

        while p1.is_some() || p2.is_some() || carry != 0 {
            let v1 = p1.as_ref().map_or(0, |n| n.val);
            let v2 = p2.as_ref().map_or(0, |n| n.val);
            let sum = v1 + v2 + carry;
            carry = sum / 10; // carry for next digit

            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.next.as_mut().unwrap();

            p1 = p1.and_then(|n| n.next); // advance pointer 1
            p2 = p2.and_then(|n| n.next); // advance pointer 2
        }

        dummy.next
    }

    /// Recursive approach.
    /// Time: O(max(m, n)), Space: O(max(m, n)) including call stack.
    pub fn add_two_numbers_recursive(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::recurse(l1, l2, 0)
    }

    fn recurse(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && carry == 0 {
            return None; // base case: nothing left to process
        }

        let v1 = l1.as_ref().map_or(0, |n| n.val);
        let v2 = l2.as_ref().map_or(0, |n| n.val);
        let sum = v1 + v2 + carry;

        let mut node = ListNode::new(sum % 10);
        node.next = Self::recurse(
            l1.and_then(|n| n.next),
            l2.and_then(|n| n.next),
            sum / 10, // propagate carry
        );

        Some(Box::new(node))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list::list_node::{from_vec, to_vec};

    #[test]
    fn test_basic() {
        // 342 + 465 = 807 => [2,4,3] + [5,6,4] = [7,0,8]
        let l1 = from_vec(&[2, 4, 3]);
        let l2 = from_vec(&[5, 6, 4]);
        assert_eq!(to_vec(Solution::add_two_numbers(l1, l2)), vec![7, 0, 8]);
    }

    #[test]
    fn test_carry_propagation() {
        // 999 + 1 = 1000 => [9,9,9] + [1] = [0,0,0,1]
        let l1 = from_vec(&[9, 9, 9]);
        let l2 = from_vec(&[1]);
        assert_eq!(
            to_vec(Solution::add_two_numbers(l1, l2)),
            vec![0, 0, 0, 1]
        );
    }

    #[test]
    fn test_different_lengths() {
        // 99 + 1 = 100 => [9,9] + [1] = [0,0,1]
        let l1 = from_vec(&[9, 9]);
        let l2 = from_vec(&[1]);
        assert_eq!(to_vec(Solution::add_two_numbers(l1, l2)), vec![0, 0, 1]);
    }

    #[test]
    fn test_both_zeros() {
        let l1 = from_vec(&[0]);
        let l2 = from_vec(&[0]);
        assert_eq!(to_vec(Solution::add_two_numbers(l1, l2)), vec![0]);
    }

    #[test]
    fn test_single_digits_with_carry() {
        // 5 + 5 = 10 => [5] + [5] = [0, 1]
        let l1 = from_vec(&[5]);
        let l2 = from_vec(&[5]);
        assert_eq!(to_vec(Solution::add_two_numbers(l1, l2)), vec![0, 1]);
    }

    // --- Recursive variant tests ---

    #[test]
    fn test_recursive_basic() {
        let l1 = from_vec(&[2, 4, 3]);
        let l2 = from_vec(&[5, 6, 4]);
        assert_eq!(
            to_vec(Solution::add_two_numbers_recursive(l1, l2)),
            vec![7, 0, 8]
        );
    }

    #[test]
    fn test_recursive_carry_propagation() {
        let l1 = from_vec(&[9, 9, 9]);
        let l2 = from_vec(&[1]);
        assert_eq!(
            to_vec(Solution::add_two_numbers_recursive(l1, l2)),
            vec![0, 0, 0, 1]
        );
    }

    #[test]
    fn test_recursive_both_zeros() {
        let l1 = from_vec(&[0]);
        let l2 = from_vec(&[0]);
        assert_eq!(
            to_vec(Solution::add_two_numbers_recursive(l1, l2)),
            vec![0]
        );
    }

    #[test]
    fn test_recursive_single_digits_with_carry() {
        let l1 = from_vec(&[5]);
        let l2 = from_vec(&[5]);
        assert_eq!(
            to_vec(Solution::add_two_numbers_recursive(l1, l2)),
            vec![0, 1]
        );
    }
}
