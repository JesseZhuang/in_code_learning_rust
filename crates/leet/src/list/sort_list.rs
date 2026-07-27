use super::list_node::ListNode;

pub struct Solution;

impl Solution {
    /// Top-down recursive merge sort. O(n log n) time, O(log n) stack space.
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Base case: 0 or 1 node
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        // Split into two halves
        let (left, right) = Self::split(head);
        let left = Self::sort_list(left);
        let right = Self::sort_list(right);
        Self::merge(left, right)
    }

    /// Bottom-up iterative merge sort. O(n log n) time, O(1) extra space.
    pub fn sort_list_bottom_up(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Count the length
        let len = Self::length(&head);
        if len <= 1 {
            return head;
        }

        let mut head = head;
        let mut size = 1;

        while size < len {
            // dummy node to build the merged result
            let mut dummy = Box::new(ListNode::new(0));
            let mut tail = &mut dummy;
            let mut cur = head;

            while cur.is_some() {
                let left = Self::cut(&mut cur, size);
                let right = Self::cut(&mut cur, size);
                let merged = Self::merge(left, right);
                tail.next = merged;
                // advance tail to end
                while tail.next.is_some() {
                    tail = tail.next.as_mut().unwrap();
                }
            }

            head = dummy.next;
            size *= 2;
        }

        head
    }

    /// Split a list into two halves using slow/fast pointer technique.
    fn split(head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        // We'll collect into a vec-like approach using pointer counting
        // Actually, let's use the length-based split for simplicity in safe Rust
        let len = Self::length(&head);
        let mid = len / 2;
        Self::cut_at(head, mid)
    }

    /// Cut the first `n` nodes from `head`, returning them. `head` is updated to the remainder.
    fn cut(head: &mut Option<Box<ListNode>>, n: usize) -> Option<Box<ListNode>> {
        let mut cur = head.take();
        let mut tail = &mut cur;
        for _ in 0..n {
            if tail.is_none() {
                break;
            }
            tail = &mut tail.as_mut().unwrap().next;
        }
        // `tail` now points to the (n+1)-th node or None
        *head = tail.take();
        cur
    }

    /// Cut a list at position `at`, returning (first `at` nodes, rest).
    fn cut_at(
        head: Option<Box<ListNode>>,
        at: usize,
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut cur = &mut dummy;
        for _ in 0..at {
            cur = cur.next.as_mut().unwrap();
        }
        let right = cur.next.take();
        let left = dummy.next;
        (left, right)
    }

    /// Merge two sorted lists into one sorted list.
    fn merge(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while l1.is_some() && l2.is_some() {
            let take_l1 = l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val;
            if take_l1 {
                let mut node = l1.take().unwrap();
                l1 = node.next.take();
                tail.next = Some(node);
            } else {
                let mut node = l2.take().unwrap();
                l2 = node.next.take();
                tail.next = Some(node);
            }
            tail = tail.next.as_mut().unwrap();
        }

        tail.next = if l1.is_some() { l1 } else { l2 };
        dummy.next
    }

    fn length(head: &Option<Box<ListNode>>) -> usize {
        let mut count = 0;
        let mut cur = head;
        while let Some(node) = cur {
            count += 1;
            cur = &node.next;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::list_node::from_vec;

    fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut out = Vec::new();
        let mut cur = head;
        while let Some(node) = cur {
            out.push(node.val);
            cur = &node.next;
        }
        out
    }

    #[test]
    fn test_sort_list_basic() {
        let head = from_vec(&[4, 2, 1, 3]);
        let result = Solution::sort_list(head);
        assert_eq!(to_vec(&result), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_sort_list_negative() {
        let head = from_vec(&[-1, 5, 3, 4, 0]);
        let result = Solution::sort_list(head);
        assert_eq!(to_vec(&result), vec![-1, 0, 3, 4, 5]);
    }

    #[test]
    fn test_sort_list_empty() {
        let result = Solution::sort_list(None);
        assert_eq!(to_vec(&result), Vec::<i32>::new());
    }

    #[test]
    fn test_sort_list_single() {
        let head = from_vec(&[1]);
        let result = Solution::sort_list(head);
        assert_eq!(to_vec(&result), vec![1]);
    }

    #[test]
    fn test_sort_list_duplicates() {
        let head = from_vec(&[3, 1, 2, 3, 1]);
        let result = Solution::sort_list(head);
        assert_eq!(to_vec(&result), vec![1, 1, 2, 3, 3]);
    }

    #[test]
    fn test_sort_list_reverse() {
        let head = from_vec(&[5, 4, 3, 2, 1]);
        let result = Solution::sort_list(head);
        assert_eq!(to_vec(&result), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_list_bottom_up_basic() {
        let head = from_vec(&[4, 2, 1, 3]);
        let result = Solution::sort_list_bottom_up(head);
        assert_eq!(to_vec(&result), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_sort_list_bottom_up_negative() {
        let head = from_vec(&[-1, 5, 3, 4, 0]);
        let result = Solution::sort_list_bottom_up(head);
        assert_eq!(to_vec(&result), vec![-1, 0, 3, 4, 5]);
    }

    #[test]
    fn test_sort_list_bottom_up_empty() {
        let result = Solution::sort_list_bottom_up(None);
        assert_eq!(to_vec(&result), Vec::<i32>::new());
    }

    #[test]
    fn test_sort_list_bottom_up_single() {
        let head = from_vec(&[1]);
        let result = Solution::sort_list_bottom_up(head);
        assert_eq!(to_vec(&result), vec![1]);
    }

    #[test]
    fn test_sort_list_bottom_up_duplicates() {
        let head = from_vec(&[3, 1, 2, 3, 1]);
        let result = Solution::sort_list_bottom_up(head);
        assert_eq!(to_vec(&result), vec![1, 1, 2, 3, 3]);
    }

    #[test]
    fn test_sort_list_bottom_up_reverse() {
        let head = from_vec(&[5, 4, 3, 2, 1]);
        let result = Solution::sort_list_bottom_up(head);
        assert_eq!(to_vec(&result), vec![1, 2, 3, 4, 5]);
    }
}
