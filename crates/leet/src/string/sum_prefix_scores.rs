/// lc 2416

use crate::structs::trie_node::TrieNode;


struct Solution {}

impl Solution {
    // only need cnt field, do not need end field in TrieNode
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut root = TrieNode::new();
        for s in &words { root.insert(&s); }
        let mut res = Vec::new();
        for s in &words {
            let (mut cur, mut sum) = (&root, 0);
            for c in s.chars() {
                cur = &cur.next[&c]; // panics if not present
                sum += cur.cnt;
            }
            res.push(sum as i32)
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use crate::string::sum_prefix_scores::Solution;

    #[test]
    fn test() {
        // stackoverflow JDemler
        macro_rules! vec_of_strings {($($x:expr),*) => (vec![$($x.into()),*])}
        let words = vec_of_strings!("abc", "ab", "bc", "b");
        // let words = ["abc", "ab", "bc", "b"].iter().map(|&s| s.into()).collect();
        assert_eq!(vec![5, 4, 3, 2], Solution::sum_prefix_scores(words));
    }
}