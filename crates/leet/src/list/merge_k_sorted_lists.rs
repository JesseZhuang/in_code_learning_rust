//! LeetCode 23 / LintCode 104, hard, tags: heap, divide and conquer, linked list.

use crate::list::list_node::ListNode;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

/// Wrap a Box<ListNode> with an Ord implementation that orders by `val` (ascending),
/// so that pushing `Reverse<Wrapped>` into a max-heap yields min-heap behavior.
struct Wrapped(Box<ListNode>);

impl PartialEq for Wrapped {
    fn eq(&self, other: &Self) -> bool {
        self.0.val == other.0.val
    }
}
impl Eq for Wrapped {}
impl PartialOrd for Wrapped {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Wrapped {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.val.cmp(&other.0.val)
    }
}

impl Solution {
    /// Min-heap of current heads. Time O(N log k), Space O(k).
    pub fn merge_k_lists_heap(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<Reverse<Wrapped>> = BinaryHeap::new();
        for list in lists {
            if let Some(node) = list {
                heap.push(Reverse(Wrapped(node)));
            }
        }
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        while let Some(Reverse(Wrapped(mut node))) = heap.pop() {
            // Detach next so we can push it back as an independent head.
            let nxt = node.next.take();
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
            if let Some(n) = nxt {
                heap.push(Reverse(Wrapped(n)));
            }
        }
        dummy.next
    }

    /// Pairwise merge bottom-up. Time O(N log k), Space O(1) extra (besides recursion stack).
    pub fn merge_k_lists_divide(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        let mut buf: Vec<Option<Box<ListNode>>> = lists;
        while buf.len() > 1 {
            let mut next_round: Vec<Option<Box<ListNode>>> = Vec::with_capacity(buf.len() / 2 + 1);
            let mut iter = buf.into_iter();
            while let Some(a) = iter.next() {
                if let Some(b) = iter.next() {
                    next_round.push(Self::merge_two(a, b));
                } else {
                    next_round.push(a);
                }
            }
            buf = next_round;
        }
        buf.into_iter().next().flatten()
    }

    fn merge_two(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let (mut a, mut b) = (list1, list2);
        while a.is_some() && b.is_some() {
            let take_a = a.as_ref().unwrap().val < b.as_ref().unwrap().val;
            let next_box = if take_a {
                let mut n = a.take().unwrap();
                a = n.next.take();
                n
            } else {
                let mut n = b.take().unwrap();
                b = n.next.take();
                n
            };
            tail.next = Some(next_box);
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = if a.is_some() { a } else { b };
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list::list_node::{from_vec, to_vec};

    fn build(input: &[Vec<i32>]) -> Vec<Option<Box<crate::list::list_node::ListNode>>> {
        input.iter().map(|v| from_vec(v)).collect()
    }

    fn verify(input: &[Vec<i32>], expected: Vec<i32>) {
        assert_eq!(to_vec(Solution::merge_k_lists_heap(build(input))), expected);
        assert_eq!(to_vec(Solution::merge_k_lists_divide(build(input))), expected);
    }

    #[test]
    fn example1() {
        verify(
            &[vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]],
            vec![1, 1, 2, 3, 4, 4, 5, 6],
        );
    }

    #[test]
    fn no_lists() {
        verify(&[], vec![]);
    }

    #[test]
    fn all_empty() {
        verify(&[vec![], vec![]], vec![]);
    }

    #[test]
    fn single_list() {
        verify(&[vec![1, 2, 3]], vec![1, 2, 3]);
    }

    #[test]
    fn empty_in_middle() {
        verify(
            &[vec![1, 3, 5], vec![], vec![2, 4, 6]],
            vec![1, 2, 3, 4, 5, 6],
        );
    }

    #[test]
    fn negative_values() {
        verify(
            &[vec![-5, 0, 5], vec![-3, 3], vec![-1, 1]],
            vec![-5, -3, -1, 0, 1, 3, 5],
        );
    }

    #[test]
    fn one_long_others_single() {
        verify(
            &[vec![1, 2, 3, 4, 5], vec![6], vec![7]],
            vec![1, 2, 3, 4, 5, 6, 7],
        );
    }

    #[test]
    fn duplicates_across_lists() {
        verify(
            &[vec![2, 2, 2], vec![2, 2], vec![2]],
            vec![2, 2, 2, 2, 2, 2],
        );
    }

    #[test]
    fn many_singletons() {
        verify(
            &[vec![5], vec![1], vec![3], vec![2], vec![4]],
            vec![1, 2, 3, 4, 5],
        );
    }
}
