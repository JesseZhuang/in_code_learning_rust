// lc 2452

use crate::structs::trie_node::TrieNode;

pub struct Solution;

impl Solution {
    /// Brute force: compare each query against every dictionary word char by char.
    /// Time O(q * d * m), Space O(1) extra.
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        queries
            .into_iter()
            .filter(|q| {
                dictionary.iter().any(|d| {
                    q.chars()
                        .zip(d.chars())
                        .filter(|(a, b)| a != b)
                        .count()
                        <= 2
                })
            })
            .collect()
    }

    /// Trie + DFS: build a trie from dictionary, then DFS each query allowing up to 2 mismatches.
    /// Time O(d * m) build + O(q * 26^2 * m) worst-case query, Space O(d * m).
    pub fn two_edit_words_trie(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        let mut root = TrieNode::new();
        for w in &dictionary {
            root.insert(w);
        }

        queries
            .into_iter()
            .filter(|q| {
                let chars: Vec<char> = q.chars().collect();
                Self::dfs(&root, &chars, 0, 0)
            })
            .collect()
    }

    fn dfs(node: &TrieNode, chars: &[char], idx: usize, edits: usize) -> bool {
        if idx == chars.len() {
            return node.end;
        }
        for (&c, child) in &node.next {
            let new_edits = edits + if c == chars[idx] { 0 } else { 1 };
            if new_edits <= 2 && Self::dfs(child, chars, idx + 1, new_edits) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|&x| x.into()).collect()
    }

    fn run(queries: &[&str], dictionary: &[&str], expected: &[&str]) {
        let mut r1 = Solution::two_edit_words(s(queries), s(dictionary));
        let mut r2 = Solution::two_edit_words_trie(s(queries), s(dictionary));
        let mut exp = s(expected);
        r1.sort();
        r2.sort();
        exp.sort();
        assert_eq!(r1, exp, "brute force");
        assert_eq!(r2, exp, "trie");
    }

    #[test]
    fn test_example1() {
        run(
            &["word", "note", "ants", "wood"],
            &["wood", "joke", "moat"],
            &["word", "note", "wood"],
        );
    }

    #[test]
    fn test_example2() {
        run(&["yes"], &["not"], &[]);
    }

    #[test]
    fn test_exact_match() {
        run(&["abc"], &["abc"], &["abc"]);
    }

    #[test]
    fn test_one_edit() {
        run(&["abc"], &["axc"], &["abc"]);
    }

    #[test]
    fn test_two_edits_boundary() {
        run(&["abc"], &["xyz", "axz"], &["abc"]);
    }

    #[test]
    fn test_three_edits_rejected() {
        run(&["abc"], &["xyz"], &[]);
    }

    #[test]
    fn test_single_char() {
        run(&["a", "b"], &["a"], &["a", "b"]);
    }
}
