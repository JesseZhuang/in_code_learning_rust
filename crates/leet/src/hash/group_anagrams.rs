use std::collections::HashMap;

/// leet 49
pub struct Solution;

impl Solution {
    /// Sort each string to form a canonical key, group by that key.
    /// Time O(n * k log k), Space O(n * k).
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
        for s in strs { // O(n)
            let mut key: Vec<u8> = s.bytes().collect();
            key.sort_unstable(); // O(k log k)
            groups.entry(key).or_default().push(s);
        }
        groups.into_values().collect()
    }

    /// Use character frequency count as key to avoid sorting.
    /// Time O(n * k), Space O(n * k).
    pub fn group_anagrams_count(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        for s in strs { // O(n)
            let mut count = [0u8; 26]; // O(1)
            for b in s.bytes() { // O(k)
                count[(b - b'a') as usize] += 1;
            }
            groups.entry(count).or_default().push(s);
        }
        groups.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize(groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = groups
            .into_iter()
            .map(|mut g| {
                g.sort();
                g
            })
            .collect();
        result.sort_by(|a, b| a[0].cmp(&b[0]));
        result
    }

    fn to_vec(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }

    fn verify(strs: &[&str], expected: Vec<Vec<&str>>) {
        let expected: Vec<Vec<String>> = expected.into_iter().map(|g| to_vec(&g)).collect();
        let input = to_vec(strs);
        assert_eq!(normalize(expected.clone()), normalize(Solution::group_anagrams(input.clone())));
        assert_eq!(normalize(expected), normalize(Solution::group_anagrams_count(input)));
    }

    #[test]
    fn test_example1() {
        verify(
            &["eat", "tea", "tan", "ate", "nat", "bat"],
            vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]],
        );
    }

    #[test]
    fn test_example2() {
        verify(&[""], vec![vec![""]]);
    }

    #[test]
    fn test_example3() {
        verify(&["a"], vec![vec!["a"]]);
    }

    #[test]
    fn test_all_same() {
        verify(&["abc", "bca", "cab"], vec![vec!["abc", "bca", "cab"]]);
    }

    #[test]
    fn test_all_different() {
        verify(&["a", "b", "c"], vec![vec!["a"], vec!["b"], vec!["c"]]);
    }

    #[test]
    fn test_single_char_anagrams() {
        verify(&["a", "a", "a"], vec![vec!["a", "a", "a"]]);
    }

    #[test]
    fn test_mixed_lengths() {
        verify(
            &["ab", "ba", "abc", "cba", "a"],
            vec![vec!["ab", "ba"], vec!["abc", "cba"], vec!["a"]],
        );
    }
}
