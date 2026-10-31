pub struct Solution;

impl Solution {
    // Remove each completed "abc" from the stack. Its remaining bytes are
    // exactly the portion of the scanned prefix that still needs reduction.
    // Each byte is pushed and removed at most once: O(n) time, O(n) space.
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len());
        for byte in s.bytes() {
            stack.push(byte);
            if stack.ends_with(b"abc") {
                stack.truncate(stack.len() - 3);
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_valid_nested_substitutions() {
        for s in ["abc", "aabcbc", "aabcbabcc", "aaabcbcbc"] {
            assert!(Solution::is_valid(s.into()), "{s}");
        }
    }

    #[test]
    fn test_valid_concatenated_substitutions() {
        assert!(Solution::is_valid("abcabcababcc".into()));
        assert!(Solution::is_valid("abcabcabc".into()));
    }

    #[test]
    fn test_invalid_order() {
        for s in ["abccba", "cababc", "acb", "bac", "aabbcc"] {
            assert!(!Solution::is_valid(s.into()), "{s}");
        }
    }

    #[test]
    fn test_leftover_characters() {
        for s in ["a", "ab", "abcab", "abca", "aabc", "abcbc"] {
            assert!(!Solution::is_valid(s.into()), "{s}");
        }
    }

    #[test]
    fn test_maximum_length_nesting() {
        let s = format!("{}{}", "a".repeat(6666), "bc".repeat(6666));
        assert!(Solution::is_valid(s));
    }
}
