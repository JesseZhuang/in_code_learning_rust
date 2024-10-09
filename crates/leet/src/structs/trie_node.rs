use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct TrieNode {
    pub next: HashMap<char, TrieNode>,
    pub end: bool,
    pub cnt: usize, // words in trie having this prefix
}

impl TrieNode {
    pub fn new() -> Self { TrieNode::default() }
    pub fn insert(&mut self, word: &str) {
        // word.chars().fold(
        //     self, |node, c| node.next.entry(c).or_default())
        //     .is_word = true
        let mut cur = self;
        for c in word.chars() {
            cur = cur.next.entry(c).or_default();
            cur.cnt += 1;
        }
        cur.end = true;
    }

    pub fn search(&self, word: &str) -> bool {
        self.get(word).map_or(false, |node| node.end)
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        self.get(prefix).is_some()
    }

    pub fn get(&self, s: &str) -> Option<&TrieNode> {
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