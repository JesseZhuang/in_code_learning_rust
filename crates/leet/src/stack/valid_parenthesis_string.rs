pub struct Solution;

impl Solution {
    /// Greedy min/max approach, O(n) time O(1) space.
    pub fn check_valid_string(s: String) -> bool {
        let mut lo = 0i32;
        let mut hi = 0i32;
        for c in s.chars() {
            match c {
                '(' => {
                    lo += 1;
                    hi += 1;
                }
                ')' => {
                    lo -= 1;
                    hi -= 1;
                }
                _ => {
                    // '*' can be '(', ')' or empty
                    lo -= 1;
                    hi += 1;
                }
            }
            if hi < 0 {
                return false;
            }
            lo = lo.max(0);
        }
        lo == 0
    }

    /// Two-pass greedy approach, O(n) time O(1) space.
    pub fn check_valid_string_two_pass(s: String) -> bool {
        let bytes = s.as_bytes();

        // Left to right: treat '*' as '('
        let mut balance = 0i32;
        for &b in bytes.iter() {
            if b == b'(' || b == b'*' {
                balance += 1;
            } else {
                balance -= 1;
            }
            if balance < 0 {
                return false;
            }
        }

        // Right to left: treat '*' as ')'
        balance = 0;
        for &b in bytes.iter().rev() {
            if b == b')' || b == b'*' {
                balance += 1;
            } else {
                balance -= 1;
            }
            if balance < 0 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example1() {
        assert!(Solution::check_valid_string("()".to_string()));
        assert!(Solution::check_valid_string_two_pass("()".to_string()));
    }

    #[test]
    fn test_example2() {
        assert!(Solution::check_valid_string("(*)".to_string()));
        assert!(Solution::check_valid_string_two_pass("(*)".to_string()));
    }

    #[test]
    fn test_example3() {
        assert!(Solution::check_valid_string("(*))".to_string()));
        assert!(Solution::check_valid_string_two_pass("(*))".to_string()));
    }

    #[test]
    fn test_empty() {
        assert!(Solution::check_valid_string("".to_string()));
        assert!(Solution::check_valid_string_two_pass("".to_string()));
    }

    #[test]
    fn test_single_star() {
        assert!(Solution::check_valid_string("*".to_string()));
        assert!(Solution::check_valid_string_two_pass("*".to_string()));
    }

    #[test]
    fn test_invalid_single_open() {
        assert!(!Solution::check_valid_string("(".to_string()));
        assert!(!Solution::check_valid_string_two_pass("(".to_string()));
    }

    #[test]
    fn test_invalid_single_close() {
        assert!(!Solution::check_valid_string(")".to_string()));
        assert!(!Solution::check_valid_string_two_pass(")".to_string()));
    }

    #[test]
    fn test_all_stars() {
        assert!(Solution::check_valid_string("***".to_string()));
        assert!(Solution::check_valid_string_two_pass("***".to_string()));
    }

    #[test]
    fn test_stars_as_empty() {
        // "()" with stars in between that act as empty
        assert!(Solution::check_valid_string("(**)".to_string()));
        assert!(Solution::check_valid_string_two_pass("(**)".to_string()));
    }

    #[test]
    fn test_complex_valid() {
        assert!(Solution::check_valid_string("(*())".to_string()));
        assert!(Solution::check_valid_string_two_pass("(*())".to_string()));
    }

    #[test]
    fn test_complex_invalid() {
        assert!(!Solution::check_valid_string("((((*))".to_string()));
        assert!(!Solution::check_valid_string_two_pass("((((*))".to_string()));
    }

    #[test]
    fn test_star_as_open() {
        // '*' acts as '(' to match the ')'
        assert!(Solution::check_valid_string("*)".to_string()));
        assert!(Solution::check_valid_string_two_pass("*)".to_string()));
    }

    #[test]
    fn test_star_as_close() {
        // '*' acts as ')' to match the '('
        assert!(Solution::check_valid_string("(*".to_string()));
        assert!(Solution::check_valid_string_two_pass("(*".to_string()));
    }

    #[test]
    fn test_unbalanced_close_early() {
        assert!(!Solution::check_valid_string(")(".to_string()));
        assert!(!Solution::check_valid_string_two_pass(")(".to_string()));
    }
}
