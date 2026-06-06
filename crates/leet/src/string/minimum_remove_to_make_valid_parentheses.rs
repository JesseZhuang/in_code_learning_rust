// LeetCode 1249: Minimum Remove to Make Valid Parentheses

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    /// Stack-based approach: track indices of unmatched parentheses
    /// Time: O(n), Space: O(n)
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut stack: Vec<usize> = Vec::new();
        let mut to_remove: HashSet<usize> = HashSet::new();
        let chars: Vec<char> = s.chars().collect();

        // First pass: identify unmatched parentheses
        for (i, &ch) in chars.iter().enumerate() {
            match ch {
                '(' => stack.push(i),
                ')' => {
                    if stack.is_empty() {
                        to_remove.insert(i);
                    } else {
                        stack.pop();
                    }
                }
                _ => {}
            }
        }

        // Add remaining unmatched '(' to removal set
        to_remove.extend(stack);

        // Build result string, skipping indices in to_remove
        chars.iter()
            .enumerate()
            .filter_map(|(i, &ch)| {
                if to_remove.contains(&i) {
                    None
                } else {
                    Some(ch)
                }
            })
            .collect()
    }

    /// Two-pass approach: left-to-right removes excess ')', right-to-left removes excess '('
    /// Time: O(n), Space: O(n)
    pub fn min_remove_to_make_valid_two_pass(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut result: Vec<Option<char>> = chars.iter().map(|&c| Some(c)).collect();

        // Left-to-right: remove unmatched ')'
        let mut open_count = 0;
        for i in 0..result.len() {
            if let Some(ch) = result[i] {
                match ch {
                    '(' => open_count += 1,
                    ')' => {
                        if open_count == 0 {
                            result[i] = None;
                        } else {
                            open_count -= 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        // Right-to-left: remove unmatched '('
        let mut close_count = 0;
        for i in (0..result.len()).rev() {
            if let Some(ch) = result[i] {
                match ch {
                    ')' => close_count += 1,
                    '(' => {
                        if close_count == 0 {
                            result[i] = None;
                        } else {
                            close_count -= 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        result.iter().filter_map(|&ch| ch).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_remove_stack() {
        // Basic case with nested valid parens
        assert_eq!(
            Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_string()),
            "lee(t(c)o)de"
        );

        // Mixed valid and invalid
        assert_eq!(
            Solution::min_remove_to_make_valid("a)b(c)d".to_string()),
            "ab(c)d"
        );

        // All invalid
        assert_eq!(
            Solution::min_remove_to_make_valid("))((".to_string()),
            ""
        );

        // Empty string
        assert_eq!(
            Solution::min_remove_to_make_valid("".to_string()),
            ""
        );

        // All valid
        assert_eq!(
            Solution::min_remove_to_make_valid("(a(b)c)".to_string()),
            "(a(b)c)"
        );

        // Only open parens
        assert_eq!(
            Solution::min_remove_to_make_valid("((((".to_string()),
            ""
        );

        // Only close parens
        assert_eq!(
            Solution::min_remove_to_make_valid("))))".to_string()),
            ""
        );

        // Nested structure - has 2 unmatched opening parens
        // Input: ( ( a ) ( b ) ( ( c ) )
        // The result keeps all matched pairs, removes the first (
        assert_eq!(
            Solution::min_remove_to_make_valid("((a)(b)((c))".to_string()),
            "(a)(b)((c))"
        );

        // Mixed invalid at start
        assert_eq!(
            Solution::min_remove_to_make_valid(")(a)(".to_string()),
            "(a)"
        );
    }

    #[test]
    fn test_min_remove_two_pass() {
        // Basic case with nested valid parens
        assert_eq!(
            Solution::min_remove_to_make_valid_two_pass("lee(t(c)o)de)".to_string()),
            "lee(t(c)o)de"
        );

        // Mixed valid and invalid
        assert_eq!(
            Solution::min_remove_to_make_valid_two_pass("a)b(c)d".to_string()),
            "ab(c)d"
        );

        // All invalid
        assert_eq!(
            Solution::min_remove_to_make_valid_two_pass("))((".to_string()),
            ""
        );

        // Empty string
        assert_eq!(
            Solution::min_remove_to_make_valid_two_pass("".to_string()),
            ""
        );

        // All valid
        assert_eq!(
            Solution::min_remove_to_make_valid_two_pass("(a(b)c)".to_string()),
            "(a(b)c)"
        );

        // Only open parens
        assert_eq!(
            Solution::min_remove_to_make_valid_two_pass("((((".to_string()),
            ""
        );

        // Only close parens
        assert_eq!(
            Solution::min_remove_to_make_valid_two_pass("))))".to_string()),
            ""
        );

        // Nested structure - has 2 unmatched opening parens
        // Input: ( ( a ) ( b ) ( ( c ) )
        // The result keeps all matched pairs, removes the first (
        assert_eq!(
            Solution::min_remove_to_make_valid_two_pass("((a)(b)((c))".to_string()),
            "(a)(b)((c))"
        );

        // Mixed invalid at start
        assert_eq!(
            Solution::min_remove_to_make_valid_two_pass(")(a)(".to_string()),
            "(a)"
        );
    }
}
