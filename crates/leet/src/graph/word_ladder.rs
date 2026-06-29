use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution;

impl Solution {
    /// BFS with wildcard pattern map.
    /// Time: O(M^2 * N) where M = word length, N = number of words
    /// Space: O(M^2 * N) for the pattern map
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let word_set: HashSet<&str> = word_list.iter().map(|s| s.as_str()).collect();
        if !word_set.contains(end_word.as_str()) {
            return 0;
        }

        // Build wildcard pattern map: pattern -> list of words
        // O(M * N) total to build, O(M) per word for M patterns
        let mut pattern_map: HashMap<String, Vec<&str>> = HashMap::new();
        for word in &word_list {
            let chars: Vec<char> = word.chars().collect();
            for i in 0..chars.len() {
                // O(M) to construct each pattern
                let pattern: String = chars[..i]
                    .iter()
                    .chain(std::iter::once(&'*'))
                    .chain(chars[i + 1..].iter())
                    .collect();
                pattern_map.entry(pattern).or_default().push(word.as_str());
            }
        }

        // BFS traversal — O(M * N) nodes visited, O(M) work per node for patterns
        let mut visited: HashSet<&str> = HashSet::new();
        let mut queue: VecDeque<(&str, i32)> = VecDeque::new();
        queue.push_back((begin_word.as_str(), 1));
        visited.insert(begin_word.as_str());

        while let Some((word, level)) = queue.pop_front() {
            let chars: Vec<char> = word.chars().collect();
            for i in 0..chars.len() {
                // O(M) per pattern construction
                let pattern: String = chars[..i]
                    .iter()
                    .chain(std::iter::once(&'*'))
                    .chain(chars[i + 1..].iter())
                    .collect();

                if let Some(neighbors) = pattern_map.get(&pattern) {
                    for &neighbor in neighbors {
                        if neighbor == end_word.as_str() {
                            return level + 1;
                        }
                        if visited.insert(neighbor) {
                            queue.push_back((neighbor, level + 1));
                        }
                    }
                }
            }
        }

        0
    }

    /// Bidirectional BFS — meets in the middle for better average performance.
    /// Time: O(M^2 * N), Space: O(M * N)
    pub fn ladder_length_bidirectional(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> i32 {
        let word_set: HashSet<String> = word_list.into_iter().collect();
        if !word_set.contains(&end_word) {
            return 0;
        }

        let mut front: HashSet<String> = HashSet::new();
        let mut back: HashSet<String> = HashSet::new();
        let mut visited: HashSet<String> = HashSet::new();

        front.insert(begin_word.clone());
        back.insert(end_word.clone());
        visited.insert(begin_word);
        visited.insert(end_word);

        let mut level = 1;

        while !front.is_empty() && !back.is_empty() {
            // Always expand the smaller frontier — O(min(|front|, |back|))
            if front.len() > back.len() {
                std::mem::swap(&mut front, &mut back);
            }

            let mut next_front: HashSet<String> = HashSet::new();

            for word in &front {
                let mut chars: Vec<u8> = word.bytes().collect();
                // O(M) positions, O(26) replacements each
                for i in 0..chars.len() {
                    let original = chars[i];
                    for c in b'a'..=b'z' {
                        if c == original {
                            continue;
                        }
                        chars[i] = c;
                        let new_word = String::from_utf8(chars.clone()).unwrap();

                        if back.contains(&new_word) {
                            return level + 1;
                        }

                        if word_set.contains(&new_word) && visited.insert(new_word.clone()) {
                            next_front.insert(new_word);
                        }
                    }
                    chars[i] = original;
                }
            }

            front = next_front;
            level += 1;
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_example1() {
        // hit -> hot -> dot -> dog -> cog = 5
        let result = Solution::ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            to_vec(&["hot", "dot", "dog", "lot", "log", "cog"]),
        );
        assert_eq!(result, 5);
    }

    #[test]
    fn test_end_word_not_in_list() {
        let result = Solution::ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            to_vec(&["hot", "dot", "dog", "lot", "log"]),
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn test_single_char() {
        // a -> c = 2
        let result = Solution::ladder_length(
            "a".to_string(),
            "c".to_string(),
            to_vec(&["a", "b", "c"]),
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn test_no_path() {
        let result = Solution::ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            to_vec(&["hot", "dot", "dog", "lot", "log", "xxx"]),
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn test_direct_neighbor() {
        // hot -> dot = 2
        let result = Solution::ladder_length(
            "hot".to_string(),
            "dot".to_string(),
            to_vec(&["dot"]),
        );
        assert_eq!(result, 2);
    }

    // Bidirectional BFS tests
    #[test]
    fn test_bidirectional_example1() {
        let result = Solution::ladder_length_bidirectional(
            "hit".to_string(),
            "cog".to_string(),
            to_vec(&["hot", "dot", "dog", "lot", "log", "cog"]),
        );
        assert_eq!(result, 5);
    }

    #[test]
    fn test_bidirectional_end_word_not_in_list() {
        let result = Solution::ladder_length_bidirectional(
            "hit".to_string(),
            "cog".to_string(),
            to_vec(&["hot", "dot", "dog", "lot", "log"]),
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn test_bidirectional_single_char() {
        let result = Solution::ladder_length_bidirectional(
            "a".to_string(),
            "c".to_string(),
            to_vec(&["a", "b", "c"]),
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn test_bidirectional_no_path() {
        let result = Solution::ladder_length_bidirectional(
            "hit".to_string(),
            "cog".to_string(),
            to_vec(&["hot", "dot", "dog", "lot", "log", "xxx"]),
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn test_bidirectional_direct_neighbor() {
        let result = Solution::ladder_length_bidirectional(
            "hot".to_string(),
            "dot".to_string(),
            to_vec(&["dot"]),
        );
        assert_eq!(result, 2);
    }
}
