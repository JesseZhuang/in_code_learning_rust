//! LeetCode 2130, medium, tags: linked list, two pointers, stack.

use crate::list::list_node::ListNode;

pub struct Solution;

impl Solution {
    /// O(n) time, O(n) space. Collect into vec, two-pointer scan from both ends.
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut vals = Vec::new();
        let mut cur = &head;
        while let Some(node) = cur {
            // O(n) — traverse once to collect
            vals.push(node.val);
            cur = &node.next;
        }
        let n = vals.len();
        let mut max_sum = 0;
        for i in 0..n / 2 {
            // O(n/2) — twin pairs: i and n-1-i
            max_sum = max_sum.max(vals[i] + vals[n - 1 - i]);
        }
        max_sum
    }

    /// O(n) time, O(n) space. Push first half onto stack, pop while scanning second half.
    pub fn pair_sum_stack(head: Option<Box<ListNode>>) -> i32 {
        // First pass: count length.
        let mut len = 0;
        let mut cur = &head;
        while let Some(node) = cur {
            // O(n)
            len += 1;
            cur = &node.next;
        }

        // Second pass: push first half onto stack, then compare with second half.
        let mut stack: Vec<i32> = Vec::with_capacity(len / 2); // O(n/2) space
        let mut cur = &head;
        for _ in 0..len / 2 {
            // O(n/2) — push first half
            if let Some(node) = cur {
                stack.push(node.val);
                cur = &node.next;
            }
        }

        let mut max_sum = 0;
        while let Some(node) = cur {
            // O(n/2) — pop matches twin in reverse order
            if let Some(twin_val) = stack.pop() {
                max_sum = max_sum.max(node.val + twin_val);
            }
            cur = &node.next;
        }
        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list::list_node::from_vec;

    fn verify(input: &[i32], expected: i32) {
        assert_eq!(
            Solution::pair_sum(from_vec(input)),
            expected,
            "vec {:?}",
            input
        );
        assert_eq!(
            Solution::pair_sum_stack(from_vec(input)),
            expected,
            "stack {:?}",
            input
        );
    }

    #[test]
    fn example1() {
        verify(&[5, 4, 2, 1], 6);
    }

    #[test]
    fn example2() {
        verify(&[4, 2, 2, 3], 7);
    }

    #[test]
    fn example3() {
        verify(&[1, 100000], 100001);
    }

    #[test]
    fn two_ones() {
        verify(&[1, 1], 2);
    }

    #[test]
    fn all_same() {
        verify(&[5, 5, 5, 5], 10);
    }

    #[test]
    fn mirror() {
        verify(&[100, 1, 1, 100], 200);
    }

    #[test]
    fn six_elements() {
        verify(&[1, 2, 3, 4, 5, 6], 7);
    }
}
