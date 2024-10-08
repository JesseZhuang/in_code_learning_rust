use crate::structs::trie_node::TrieNode;

// lc 208

#[derive(Debug, Default)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, word: String) {
        self.root.insert(&word);
    }

    fn search(&self, word: String) -> bool {
        self.root.search(&word)
    }

    fn starts_with(&self, prefix: &str) -> bool {
        self.root.starts_with(&prefix)
    }
}