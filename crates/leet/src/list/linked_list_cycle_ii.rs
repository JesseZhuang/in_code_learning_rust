//! LeetCode 142 - Linked List Cycle II

pub struct Solution;

/// Simple node for this problem (raw-pointer based to allow cycles).
#[derive(Debug)]
pub struct CycleListNode {
    pub val: i32,
    pub next: *mut CycleListNode,
}

impl Solution {
    /// Floyd's tortoise and hare. O(n) time, O(1) space.
    pub fn detect_cycle(head: *mut CycleListNode) -> *mut CycleListNode {
        unsafe {
            let (mut slow, mut fast) = (head, head);
            loop {
                if fast.is_null() || (*fast).next.is_null() {
                    return std::ptr::null_mut();
                }
                slow = (*slow).next;
                fast = (*(*fast).next).next;
                if slow == fast {
                    break;
                }
            }
            slow = head;
            while slow != fast {
                slow = (*slow).next;
                fast = (*fast).next;
            }
            slow
        }
    }

    /// HashSet approach. O(n) time, O(n) space.
    pub fn detect_cycle_hash(head: *mut CycleListNode) -> *mut CycleListNode {
        unsafe {
            let mut seen = std::collections::HashSet::new();
            let mut cur = head;
            while !cur.is_null() {
                if !seen.insert(cur) {
                    return cur;
                }
                cur = (*cur).next;
            }
            std::ptr::null_mut()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_cycle_list(values: &[i32], pos: i32) -> (Vec<Box<CycleListNode>>, *mut CycleListNode) {
        if values.is_empty() {
            return (vec![], std::ptr::null_mut());
        }
        let mut nodes: Vec<Box<CycleListNode>> = values
            .iter()
            .map(|&v| {
                Box::new(CycleListNode {
                    val: v,
                    next: std::ptr::null_mut(),
                })
            })
            .collect();
        let mut ptrs: Vec<*mut CycleListNode> = nodes
            .iter_mut()
            .map(|n| &mut **n as *mut CycleListNode)
            .collect();
        unsafe {
            for i in 0..ptrs.len() - 1 {
                (*ptrs[i]).next = ptrs[i + 1];
            }
            if pos >= 0 {
                let last_idx = ptrs.len() - 1;
                (*ptrs[last_idx]).next = ptrs[pos as usize];
            }
        }
        let head = ptrs[0];
        (nodes, head)
    }

    fn expected_node(nodes: &[Box<CycleListNode>], pos: i32) -> *mut CycleListNode {
        if pos < 0 {
            return std::ptr::null_mut();
        }
        &*nodes[pos as usize] as *const CycleListNode as *mut CycleListNode
    }

    #[test]
    fn example1_cycle_at_1() {
        let (nodes, head) = build_cycle_list(&[3, 2, 0, -4], 1);
        assert_eq!(Solution::detect_cycle(head), expected_node(&nodes, 1));
        assert_eq!(Solution::detect_cycle_hash(head), expected_node(&nodes, 1));
    }

    #[test]
    fn example2_cycle_at_0() {
        let (nodes, head) = build_cycle_list(&[1, 2], 0);
        assert_eq!(Solution::detect_cycle(head), expected_node(&nodes, 0));
        assert_eq!(Solution::detect_cycle_hash(head), expected_node(&nodes, 0));
    }

    #[test]
    fn example3_no_cycle() {
        let (nodes, head) = build_cycle_list(&[1], -1);
        let _ = nodes;
        assert_eq!(Solution::detect_cycle(head), std::ptr::null_mut());
        assert_eq!(Solution::detect_cycle_hash(head), std::ptr::null_mut());
    }

    #[test]
    fn empty_list() {
        assert_eq!(
            Solution::detect_cycle(std::ptr::null_mut()),
            std::ptr::null_mut()
        );
        assert_eq!(
            Solution::detect_cycle_hash(std::ptr::null_mut()),
            std::ptr::null_mut()
        );
    }

    #[test]
    fn single_node_self_cycle() {
        let (nodes, head) = build_cycle_list(&[1], 0);
        assert_eq!(Solution::detect_cycle(head), expected_node(&nodes, 0));
        assert_eq!(Solution::detect_cycle_hash(head), expected_node(&nodes, 0));
    }

    #[test]
    fn long_tail_short_cycle() {
        let (nodes, head) = build_cycle_list(&[1, 2, 3, 4, 5], 3);
        assert_eq!(Solution::detect_cycle(head), expected_node(&nodes, 3));
        assert_eq!(Solution::detect_cycle_hash(head), expected_node(&nodes, 3));
    }

    #[test]
    fn cycle_at_last_node() {
        let (nodes, head) = build_cycle_list(&[1, 2, 3], 2);
        assert_eq!(Solution::detect_cycle(head), expected_node(&nodes, 2));
        assert_eq!(Solution::detect_cycle_hash(head), expected_node(&nodes, 2));
    }
}
