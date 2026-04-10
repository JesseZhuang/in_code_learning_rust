// leet code 3

use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // sliding window with hash map, O(n) time, O(min(n,m)) space
        let mut map: HashMap<char, usize> = HashMap::new(); // char -> last seen index
        let mut max_len = 0;
        let mut left = 0; // left boundary of window

        for (right, ch) in s.chars().enumerate() {
            // O(n) — each char visited once by right pointer
            if let Some(&prev_idx) = map.get(&ch) {
                // jump left past the previous occurrence
                left = left.max(prev_idx + 1);
            }
            map.insert(ch, right); // update last seen index
            max_len = max_len.max(right - left + 1); // update answer
        }

        max_len as i32
    }

    pub fn length_of_longest_substring_set(s: String) -> i32 {
        // sliding window with hash set, O(n) time, O(min(n,m)) space
        let mut set: HashSet<char> = HashSet::new();
        let chars: Vec<char> = s.chars().collect();
        let mut max_len = 0;
        let mut left = 0; // left boundary of window

        for right in 0..chars.len() {
            // O(n) amortized — left moves at most n times total
            while set.contains(&chars[right]) {
                set.remove(&chars[left]); // shrink window one step
                left += 1;
            }
            set.insert(chars[right]);
            max_len = max_len.max(right - left + 1); // update answer
        }

        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);

        assert_eq!(Solution::length_of_longest_substring_set("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring_set("bbbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring_set("pwwkew".to_string()), 3);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
        assert_eq!(Solution::length_of_longest_substring_set("".to_string()), 0);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::length_of_longest_substring("a".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring_set("a".to_string()), 1);
    }

    #[test]
    fn test_all_unique() {
        assert_eq!(Solution::length_of_longest_substring("abcde".to_string()), 5);
        assert_eq!(Solution::length_of_longest_substring_set("abcde".to_string()), 5);
    }

    #[test]
    fn test_duplicate_at_edges() {
        assert_eq!(Solution::length_of_longest_substring("abca".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring_set("abca".to_string()), 3);
    }
}
