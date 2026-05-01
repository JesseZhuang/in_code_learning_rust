// LeetCode 3043, medium, tags: array, hash table, trie, string.

use std::collections::HashMap;

pub struct Solution;

struct TrieNode {
    children: HashMap<u8, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, word: &[u8]) {
        let mut cur = self;
        for &c in word {
            cur = cur.children.entry(c).or_insert_with(TrieNode::new);
        }
    }

    fn lcp_len(&self, word: &[u8]) -> i32 {
        let mut cur = self;
        let mut res = 0;
        for &c in word {
            match cur.children.get(&c) {
                Some(next) => {
                    cur = next;
                    res += 1;
                }
                None => break,
            }
        }
        res
    }
}

impl Solution {
    /// O(m*d1+n*d2) time, O(m*d1) space.
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut root = TrieNode::new();
        for a in &arr1 {
            let s = a.to_string();
            root.insert(s.as_bytes());
        }
        let mut res = 0;
        for a in &arr2 {
            let s = a.to_string();
            res = res.max(root.lcp_len(s.as_bytes()));
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            3,
            Solution::longest_common_prefix(vec![1, 10, 100], vec![1000])
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            0,
            Solution::longest_common_prefix(vec![1, 2, 3], vec![4, 4, 4])
        );
    }
}
