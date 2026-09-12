//! LeetCode 234, easy, tags: linked list, two pointers, stack, recursion.

use crate::list::list_node::{from_vec, ListNode};

pub struct Solution;

impl Solution {
    /// O(n) time, O(n) space. Collect values then compare with two pointers.
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut vals = Vec::new();
        let mut cur = &head;
        while let Some(node) = cur { // O(n)
            vals.push(node.val);
            cur = &node.next;
        }
        let (mut l, mut r) = (0, vals.len());
        while l < r { // O(n/2)
            r -= 1;
            if vals[l] != vals[r] {
                return false;
            }
            l += 1;
        }
        true
    }

    /// O(n) time, O(n) space. Stack-based: push first half, compare with second half.
    pub fn is_palindrome_stack(head: Option<Box<ListNode>>) -> bool {
        let vals: Vec<i32> = {
            let mut v = Vec::new();
            let mut cur = &head;
            while let Some(node) = cur {
                v.push(node.val);
                cur = &node.next;
            }
            v
        };
        let n = vals.len();
        let mut stack: Vec<i32> = vals[..n / 2].to_vec(); // O(n/2)
        let start = if n % 2 == 0 { n / 2 } else { n / 2 + 1 }; // odd: skip middle
        for &v in &vals[start..] { // O(n/2)
            if stack.pop() != Some(v) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list::list_node::from_vec;

    fn verify(input: &[i32], expected: bool) {
        assert_eq!(
            Solution::is_palindrome(from_vec(input)),
            expected,
            "two_pointer {:?}",
            input
        );
        assert_eq!(
            Solution::is_palindrome_stack(from_vec(input)),
            expected,
            "stack {:?}",
            input
        );
    }

    #[test]
    fn even_palindrome() {
        verify(&[1, 2, 2, 1], true);
    }

    #[test]
    fn not_palindrome() {
        verify(&[1, 2], false);
    }

    #[test]
    fn single_node() {
        verify(&[1], true);
    }

    #[test]
    fn odd_palindrome() {
        verify(&[1, 2, 1], true);
    }

    #[test]
    fn odd_not_palindrome() {
        verify(&[1, 2, 3], false);
    }

    #[test]
    fn all_same() {
        verify(&[5, 5, 5, 5], true);
    }

    #[test]
    fn two_same() {
        verify(&[1, 1], true);
    }

    #[test]
    fn long_palindrome() {
        let mut vals: Vec<i32> = (1..=50).collect();
        vals.extend((1..=50).rev());
        verify(&vals, true);
    }

    #[test]
    fn boundary_values() {
        verify(&[0, 9, 0], true);
        verify(&[0, 9, 1], false);
    }
}
