/// LeetCode 1804

use std::collections::HashMap;

#[derive(Default)]
pub struct TrieII {
    next: HashMap<char, TrieII>,
    word_count: i32,
    prefix_count: i32,
}

impl TrieII {
    pub fn new() -> Self { TrieII::default() }

    pub fn insert(&mut self, word: &str) {
        let mut cur = self;
        for c in word.chars() {
            cur = cur.next.entry(c).or_default();
            cur.prefix_count += 1;
        }
        cur.word_count += 1;
    }

    pub fn count_words_equal_to(&self, word: &str) -> i32 {
        self.get(word).map_or(0, |n| n.word_count)
    }

    pub fn count_words_starting_with(&self, prefix: &str) -> i32 {
        self.get(prefix).map_or(0, |n| n.prefix_count)
    }

    pub fn erase(&mut self, word: &str) {
        let mut cur = self;
        for c in word.chars() {
            cur = cur.next.get_mut(&c).unwrap();
            cur.prefix_count -= 1;
        }
        cur.word_count -= 1;
    }

    fn get(&self, word: &str) -> Option<&TrieII> {
        let mut cur = self;
        for c in word.chars() {
            match cur.next.get(&c) {
                Some(node) => cur = node,
                None => return None,
            }
        }
        Some(cur)
    }
}

#[cfg(test)]
mod tests {
    use crate::string::trie_ii::TrieII;

    #[test]
    fn test_trie_ii() {
        let mut trie = TrieII::new();
        trie.insert("apple");
        trie.insert("apple");
        assert_eq!(2, trie.count_words_equal_to("apple"));
        assert_eq!(2, trie.count_words_starting_with("app"));
        trie.erase("apple");
        assert_eq!(1, trie.count_words_equal_to("apple"));
        assert_eq!(1, trie.count_words_starting_with("app"));
        trie.erase("apple");
        assert_eq!(0, trie.count_words_starting_with("app"));
    }
}
