//! LeetCode 25, hard, tags: linked list, recursion.

use crate::list::list_node::ListNode;

pub struct Solution;

impl Solution {
    /// Iterative approach. Time O(n), Space O(1).
    ///
    /// Uses a dummy head and a raw pointer to track the tail of the processed
    /// portion. For each group of k nodes, reverses them in-place.
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k <= 1 {
            return head;
        }
        let k = k as usize;

        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut group_prev: *mut ListNode = &mut *dummy;

        'outer: loop {
            // Check if k nodes remain after group_prev.
            let mut scout = unsafe { &*group_prev }.next.as_deref();
            for _ in 0..k {
                match scout {
                    Some(n) => scout = n.next.as_deref(),
                    None => break 'outer,
                }
            }

            // Reverse k nodes starting after group_prev.
            // Save a pointer to the first node (will become the group tail after reversal).
            let mut cur = unsafe { &mut *group_prev }.next.take();
            let first_ptr: *mut ListNode = cur.as_mut().unwrap().as_mut();

            let mut prev: Option<Box<ListNode>> = None;
            for _ in 0..k {
                let mut node = cur.unwrap();
                cur = node.next.take();
                node.next = prev;
                prev = Some(node);
            }

            // Attach reversed group after group_prev.
            unsafe { &mut *group_prev }.next = prev;
            // Attach remainder after the new tail (original first node).
            unsafe { &mut *first_ptr }.next = cur;
            // Advance group_prev to the new tail.
            group_prev = first_ptr;
        }

        dummy.next
    }

    /// Recursive approach. Time O(n), Space O(n/k) recursion stack.
    ///
    /// Counts k nodes; if available, reverses them and recurses on the remainder.
    pub fn reverse_k_group_recursive(
        head: Option<Box<ListNode>>,
        k: i32,
    ) -> Option<Box<ListNode>> {
        if k <= 1 {
            return head;
        }
        let k = k as usize;

        // Check if there are at least k nodes.
        let mut scout = head.as_deref();
        for _ in 0..k {
            match scout {
                Some(n) => scout = n.next.as_deref(),
                None => return head, // fewer than k nodes, return as-is
            }
        }

        // Reverse the first k nodes.
        let mut prev: Option<Box<ListNode>> = None;
        let mut cur = head;
        for _ in 0..k {
            let mut node = cur.unwrap();
            cur = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        // Walk to the tail of the reversed segment (the original first node).
        let mut tail = prev.as_mut().unwrap();
        while tail.next.is_some() {
            tail = tail.next.as_mut().unwrap();
        }
        // Recurse on the remainder and attach.
        tail.next = Self::reverse_k_group_recursive(cur, k as i32);

        prev
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list::list_node::{from_vec, to_vec};

    fn verify(input: &[i32], k: i32, expected: &[i32]) {
        assert_eq!(
            to_vec(Solution::reverse_k_group(from_vec(input), k)),
            expected.to_vec(),
            "iterative input={:?} k={}",
            input,
            k
        );
        assert_eq!(
            to_vec(Solution::reverse_k_group_recursive(from_vec(input), k)),
            expected.to_vec(),
            "recursive input={:?} k={}",
            input,
            k
        );
    }

    #[test]
    fn example1() {
        verify(&[1, 2, 3, 4, 5], 2, &[2, 1, 4, 3, 5]);
    }

    #[test]
    fn example2() {
        verify(&[1, 2, 3, 4, 5], 3, &[3, 2, 1, 4, 5]);
    }

    #[test]
    fn k_equals_one() {
        verify(&[1, 2, 3, 4, 5], 1, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn k_equals_length() {
        verify(&[1, 2, 3, 4, 5], 5, &[5, 4, 3, 2, 1]);
    }

    #[test]
    fn single_node() {
        verify(&[42], 2, &[42]);
    }

    #[test]
    fn empty_list() {
        verify(&[], 3, &[]);
    }

    #[test]
    fn two_full_groups() {
        verify(&[1, 2, 3, 4], 2, &[2, 1, 4, 3]);
    }

    #[test]
    fn two_full_groups_of_three() {
        verify(&[1, 2, 3, 4, 5, 6], 3, &[3, 2, 1, 6, 5, 4]);
    }
}
