pub struct Solution;

impl Solution {
    /// Stack approach, O(n) time O(n) space.
    pub fn longest_valid_parentheses_stack(s: String) -> i32 {
        let mut stack: Vec<i32> = vec![-1]; // base index
        let mut max_len = 0i32;

        // O(n) — one pass through the string
        for (i, c) in s.chars().enumerate() {
            let i = i as i32;
            if c == '(' {
                stack.push(i);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i); // new base
                } else {
                    max_len = max_len.max(i - stack.last().unwrap());
                }
            }
        }
        max_len
    }

    /// Two-pass greedy approach, O(n) time O(1) space.
    pub fn longest_valid_parentheses_two_pass(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut max_len = 0i32;

        // Left to right pass — O(n)
        let (mut open, mut close) = (0i32, 0i32);
        for &b in bytes.iter() {
            if b == b'(' {
                open += 1;
            } else {
                close += 1;
            }
            if open == close {
                max_len = max_len.max(open + close);
            } else if close > open {
                open = 0;
                close = 0;
            }
        }

        // Right to left pass — O(n)
        open = 0;
        close = 0;
        for &b in bytes.iter().rev() {
            if b == b'(' {
                open += 1;
            } else {
                close += 1;
            }
            if open == close {
                max_len = max_len.max(open + close);
            } else if open > close {
                open = 0;
                close = 0;
            }
        }

        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn check(s: &str, expected: i32) {
        assert_eq!(
            Solution::longest_valid_parentheses_stack(s.to_string()),
            expected,
            "stack failed for {:?}",
            s
        );
        assert_eq!(
            Solution::longest_valid_parentheses_two_pass(s.to_string()),
            expected,
            "two_pass failed for {:?}",
            s
        );
    }

    #[test]
    fn test_example1() {
        check("(()", 2);
    }

    #[test]
    fn test_example2() {
        check(")()())", 4);
    }

    #[test]
    fn test_empty() {
        check("", 0);
    }

    #[test]
    fn test_single_open() {
        check("(", 0);
    }

    #[test]
    fn test_single_close() {
        check(")", 0);
    }

    #[test]
    fn test_all_open() {
        check("((((", 0);
    }

    #[test]
    fn test_consecutive_pairs() {
        check("()()", 4);
    }

    #[test]
    fn test_three_pairs() {
        check("()()()", 6);
    }

    #[test]
    fn test_nested() {
        check("((()))", 6);
    }

    #[test]
    fn test_mixed() {
        check("()(())", 6);
    }

    #[test]
    fn test_complex() {
        check(")(())()(()))(", 10);
    }

    #[test]
    fn test_trailing_open() {
        check("()()()(", 6);
    }
}
