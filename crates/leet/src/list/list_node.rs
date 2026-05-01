//! Standard LeetCode singly-linked-list node, matching the rust playground signature.
//!
//! `Option<Box<ListNode>>` gives us:
//! - `Option`: a node may or may not exist (terminating None ≈ null in Java/C++).
//! - `Box`: heap-owned, allowing recursive struct definition with known size.

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// Build a list from a slice. Useful for tests.
pub fn from_vec(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &v in values.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

/// Collect a list into a Vec. Useful for tests.
pub fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    while let Some(node) = head {
        out.push(node.val);
        head = node.next;
    }
    out
}
