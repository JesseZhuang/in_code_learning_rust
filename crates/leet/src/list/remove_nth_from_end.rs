//! LeetCode 19, medium, tags: linked list, two pointers.

use crate::list::list_node::ListNode;

pub struct Solution;

impl Solution {
    /// Two pointers with n-gap. Time O(n), Space O(1).
    ///
    /// Advance `fast` n steps ahead of `slow`, then move both until fast
    /// reaches the last node. `slow` will be just before the node to remove.
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        // Safety: we only read through fast, and mutate through slow on disjoint nodes.
        let mut fast = &*dummy as *const ListNode;
        // Advance fast n steps
        for _ in 0..n {
            unsafe {
                fast = (*fast).next.as_deref().unwrap();
            }
        }
        let mut slow = &mut *dummy as *mut ListNode;
        // Move both until fast is at the last node
        unsafe {
            while (*fast).next.is_some() {
                fast = (*fast).next.as_deref().unwrap();
                slow = (*slow).next.as_deref_mut().unwrap();
            }
            // slow.next is the node to remove
            let removed = (*slow).next.take();
            (*slow).next = removed.and_then(|n| n.next);
        }
        dummy.next
    }

    /// Single pass counting approach. Time O(n), Space O(1).
    ///
    /// Count the length, then remove the (len - n)th node from the beginning.
    pub fn remove_nth_from_end_v2(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut len = 0;
        {
            let mut cur = dummy.next.as_deref();
            while let Some(node) = cur {
                len += 1;
                cur = node.next.as_deref();
            }
        }
        let target = len - n;
        let mut cur = &mut *dummy as &mut ListNode;
        for _ in 0..target {
            cur = cur.next.as_deref_mut().unwrap();
        }
        let removed = cur.next.take();
        cur.next = removed.and_then(|n| n.next);
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list::list_node::{from_vec, to_vec};

    #[test]
    fn example1() {
        let head = from_vec(&[1, 2, 3, 4, 5]);
        assert_eq!(to_vec(Solution::remove_nth_from_end(head, 2)), vec![1, 2, 3, 5]);
    }

    #[test]
    fn single_node() {
        let head = from_vec(&[1]);
        assert_eq!(to_vec(Solution::remove_nth_from_end(head, 1)), vec![]);
    }

    #[test]
    fn two_nodes_remove_last() {
        let head = from_vec(&[1, 2]);
        assert_eq!(to_vec(Solution::remove_nth_from_end(head, 1)), vec![1]);
    }

    #[test]
    fn two_nodes_remove_first() {
        let head = from_vec(&[1, 2]);
        assert_eq!(to_vec(Solution::remove_nth_from_end(head, 2)), vec![2]);
    }

    #[test]
    fn example1_v2() {
        let head = from_vec(&[1, 2, 3, 4, 5]);
        assert_eq!(to_vec(Solution::remove_nth_from_end_v2(head, 2)), vec![1, 2, 3, 5]);
    }

    #[test]
    fn single_node_v2() {
        let head = from_vec(&[1]);
        assert_eq!(to_vec(Solution::remove_nth_from_end_v2(head, 1)), vec![]);
    }

    #[test]
    fn two_nodes_remove_last_v2() {
        let head = from_vec(&[1, 2]);
        assert_eq!(to_vec(Solution::remove_nth_from_end_v2(head, 1)), vec![1]);
    }

    #[test]
    fn two_nodes_remove_first_v2() {
        let head = from_vec(&[1, 2]);
        assert_eq!(to_vec(Solution::remove_nth_from_end_v2(head, 2)), vec![2]);
    }
}
