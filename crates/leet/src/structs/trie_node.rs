use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct TrieNode {
    next: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self { TrieNode::default() }
    pub(crate) fn insert(&mut self, word: String) {
        // word.chars().fold(
        //     self, |node, c| node.next.entry(c).or_default())
        //     .is_word = true
        let mut cur = self;
        for c in word.chars() {
            cur = cur.next.entry(c).or_default();
        }
        cur.is_word = true
    }

    pub(crate) fn search(&self, word: String) -> bool {
        self.get(word).map_or(false, |node| node.is_word)
    }

    pub(crate) fn starts_with(&self, prefix: String) -> bool {
        self.get(prefix).is_some()
    }

    fn get(&self, s: String) -> Option<&TrieNode> {
        // s.chars().try_fold(self, |node, c| node.next.get(&c))
        let mut cur = self;
        for c in s.chars() {
            match cur.next.get(&c) {
                Some(node) => cur = node,
                None => return None
            }
        }
        Some(cur)
    }
}