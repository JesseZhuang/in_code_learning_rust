/// lc 211, medium
///
/// Design a data structure that supports adding words and searching with '.' wildcards.
/// Uses a HashMap-based trie where each node maps char -> child node.

use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>, // O(1) lookup per character
    is_end: bool,
}

struct WordDictionary {
    root: TrieNode,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            root: TrieNode::default(),
        }
    }

    /// Adds a word to the trie.
    /// Time: O(L) where L = word length. Space: O(L) for new nodes.
    fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_default(); // O(1) per char
        }
        node.is_end = true;
    }

    /// Searches for a word, where '.' matches any single character.
    /// Time: O(26^D * L) worst case where D = number of dots, L = word length.
    /// Space: O(L) recursion stack.
    fn search(&self, word: String) -> bool {
        Self::dfs(&self.root, word.as_bytes(), 0)
    }

    fn dfs(node: &TrieNode, word: &[u8], i: usize) -> bool {
        if i == word.len() {
            return node.is_end; // only true if a complete word ends here
        }
        let ch = word[i] as char;
        if ch == '.' {
            // Wildcard: try every child branch — O(26) branching factor
            for child in node.children.values() {
                if Self::dfs(child, word, i + 1) {
                    return true;
                }
            }
            false
        } else {
            // Exact match: follow single child — O(1)
            match node.children.get(&ch) {
                Some(child) => Self::dfs(child, word, i + 1),
                None => false,
            }
        }
    }
}

pub struct Solution;

impl Solution {
    // Wrapper to expose WordDictionary through Solution for consistency.
    pub fn run_example() -> bool {
        let mut dict = WordDictionary::new();
        dict.add_word("bad".to_string());
        dict.add_word("dad".to_string());
        dict.add_word("mad".to_string());
        !dict.search("pad".to_string())
            && dict.search("bad".to_string())
            && dict.search(".ad".to_string())
            && dict.search("b..".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        // LC example: add bad/dad/mad, search pad=false, bad=true, .ad=true, b..=true
        let mut dict = WordDictionary::new();
        dict.add_word("bad".to_string());
        dict.add_word("dad".to_string());
        dict.add_word("mad".to_string());
        assert!(!dict.search("pad".to_string()));
        assert!(dict.search("bad".to_string()));
        assert!(dict.search(".ad".to_string()));
        assert!(dict.search("b..".to_string()));
    }

    #[test]
    fn test_empty_search() {
        // Empty string was never added, so search returns false.
        let dict = WordDictionary::new();
        assert!(!dict.search("".to_string()));
    }

    #[test]
    fn test_empty_word_added() {
        // Adding empty string makes empty search return true.
        let mut dict = WordDictionary::new();
        dict.add_word("".to_string());
        assert!(dict.search("".to_string()));
    }

    #[test]
    fn test_single_char() {
        let mut dict = WordDictionary::new();
        dict.add_word("a".to_string());
        assert!(dict.search("a".to_string()));
        assert!(!dict.search("b".to_string()));
        assert!(dict.search(".".to_string())); // dot matches 'a'
    }

    #[test]
    fn test_all_dots() {
        // "..." should match any 3-letter word
        let mut dict = WordDictionary::new();
        dict.add_word("abc".to_string());
        assert!(dict.search("...".to_string()));
        assert!(!dict.search("....".to_string())); // length mismatch
        assert!(!dict.search("..".to_string()));
    }

    #[test]
    fn test_prefix_not_word() {
        // "ba" is a prefix of "bad" but was never added as a word
        let mut dict = WordDictionary::new();
        dict.add_word("bad".to_string());
        assert!(!dict.search("ba".to_string()));
        assert!(!dict.search("b.".to_string()));
        assert!(dict.search("bad".to_string()));
    }

    #[test]
    fn test_dot_in_middle() {
        let mut dict = WordDictionary::new();
        dict.add_word("hello".to_string());
        dict.add_word("hallo".to_string());
        assert!(dict.search("h.llo".to_string()));  // matches both
        assert!(dict.search("he.lo".to_string()));   // matches hello
        assert!(!dict.search("h.lp.".to_string()));  // no match
    }
}
