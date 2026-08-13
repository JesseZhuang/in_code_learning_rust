pub struct Solution;

impl Solution {
    /// O(n) time, O(1) space. Three-tier cache: res, last, cur.
    pub fn calculate(s: String) -> i32 {
        let s = s + "##";
        let (mut res, mut last, mut cur) = (0i64, 0i64, 0i64);
        let mut prev_op = '+';
        for c in s.chars() {
            if c.is_whitespace() {
                continue;
            }
            if c.is_ascii_digit() {
                cur = cur * 10 + (c as i64 - '0' as i64); // O(1) per digit
            } else {
                match prev_op {
                    '*' => last *= cur,        // fold cur into last
                    '/' => last /= cur,        // truncate toward zero
                    '+' => { res += last; last = cur; }
                    _ => { res += last; last = -cur; } // '-'
                }
                prev_op = c;
                cur = 0;
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::calculate("3+2*2".to_string()), 7);
        assert_eq!(Solution::calculate(" 3/2 ".to_string()), 1);
        assert_eq!(Solution::calculate(" 3+5 / 2 ".to_string()), 5);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(Solution::calculate("1-1+1".to_string()), 1);
        assert_eq!(Solution::calculate("2147483647".to_string()), 2147483647);
        assert_eq!(Solution::calculate("0".to_string()), 0);
        assert_eq!(Solution::calculate("1+1+1+1+1".to_string()), 5);
        assert_eq!(Solution::calculate("14-3/2".to_string()), 13);
    }

    #[test]
    fn test_all_operators() {
        assert_eq!(Solution::calculate("2*3+4".to_string()), 10);
        assert_eq!(Solution::calculate("2+3*4".to_string()), 14);
        assert_eq!(Solution::calculate("10-2*3+1".to_string()), 5);
        assert_eq!(Solution::calculate("100/10/2".to_string()), 5);
    }
}
