//! LeetCode 21 / LintCode 165, easy, tags: linked list, recursion.

use crate::list::list_node::ListNode;

pub struct Solution;

impl Solution {
    /// Iterative splice. Time O(n + m), Space O(1).
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let (mut a, mut b) = (list1, list2);
        while a.is_some() && b.is_some() {
            // Borrow values without consuming the boxes.
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

    #[test]
    fn example1() {
        let a = from_vec(&[1, 2, 4]);
        let b = from_vec(&[1, 3, 4]);
        let m = Solution::merge_two_lists(a, b);
        assert_eq!(to_vec(m), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn both_empty() {
        let m = Solution::merge_two_lists(None, None);
        assert_eq!(to_vec(m), Vec::<i32>::new());
    }

    #[test]
    fn one_empty() {
        let b = from_vec(&[0]);
        let m = Solution::merge_two_lists(None, b);
        assert_eq!(to_vec(m), vec![0]);
    }

    #[test]
    fn left_smaller_all() {
        let a = from_vec(&[1, 2, 3]);
        let b = from_vec(&[4, 5, 6]);
        let m = Solution::merge_two_lists(a, b);
        assert_eq!(to_vec(m), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn interleaved() {
        let a = from_vec(&[-5, 0, 7]);
        let b = from_vec(&[-3, 4, 8]);
        let m = Solution::merge_two_lists(a, b);
        assert_eq!(to_vec(m), vec![-5, -3, 0, 4, 7, 8]);
    }

    #[test]
    fn duplicates_across() {
        let a = from_vec(&[1, 1, 1]);
        let b = from_vec(&[1, 1]);
        let m = Solution::merge_two_lists(a, b);
        assert_eq!(to_vec(m), vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn one_long_one_short() {
        let a = from_vec(&[1]);
        let b = from_vec(&[2, 3, 4, 5, 6]);
        let m = Solution::merge_two_lists(a, b);
        assert_eq!(to_vec(m), vec![1, 2, 3, 4, 5, 6]);
    }
}
