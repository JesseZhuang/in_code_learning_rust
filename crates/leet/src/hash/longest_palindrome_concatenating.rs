use std::collections::HashMap;

pub struct Solution;

impl Solution {
    // lc 2131, greedy + hash, O(n) time, O(n) space.
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut freq: HashMap<&str, i32> = HashMap::new();
        for w in &words {
            *freq.entry(w.as_str()).or_insert(0) += 1;
        }

        let mut length = 0i32;
        let mut has_center = false;

        for (word, &count) in &freq {
            let bytes = word.as_bytes();
            if bytes[0] == bytes[1] {
                // Palindromic word
                length += (count / 2) * 4;
                if count % 2 == 1 {
                    has_center = true;
                }
            } else if bytes[0] < bytes[1] {
                // Non-palindromic: only process once per pair (lexicographic order)
                let rev: String = word.chars().rev().collect();
                if let Some(&rev_count) = freq.get(rev.as_str()) {
                    length += count.min(rev_count) * 4;
                }
            }
        }

        if has_center {
            length += 2;
        }

        length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_words(s: &[&str]) -> Vec<String> {
        s.iter().map(|&w| w.to_string()).collect()
    }

    #[test]
    fn test_example1() {
        assert_eq!(Solution::longest_palindrome(to_words(&["lc", "cl", "gg"])), 6);
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::longest_palindrome(to_words(&["ab", "ty", "yt", "lc", "cl", "ab"])),
            8
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::longest_palindrome(to_words(&["cc", "ll", "xx"])), 2);
    }

    #[test]
    fn test_all_palindromic_pairs() {
        assert_eq!(Solution::longest_palindrome(to_words(&["aa", "aa", "bb", "bb"])), 8);
    }

    #[test]
    fn test_palindromic_with_center() {
        assert_eq!(Solution::longest_palindrome(to_words(&["aa", "aa", "aa"])), 6);
    }

    #[test]
    fn test_no_pairs() {
        assert_eq!(Solution::longest_palindrome(to_words(&["ab", "cd", "ef"])), 0);
    }

    #[test]
    fn test_single_palindromic() {
        assert_eq!(Solution::longest_palindrome(to_words(&["aa"])), 2);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::longest_palindrome(vec![]), 0);
    }

    #[test]
    fn test_mixed() {
        // "ab"+"ba" pair (4) + "cc","cc" pair (4) + "dd" center (2) = 10
        assert_eq!(
            Solution::longest_palindrome(to_words(&["ab", "ba", "cc", "cc", "dd"])),
            10
        );
    }
}
