pub struct Solution;

impl Solution {
    /// Stack approach. O(n) time, O(n) space.
    /// On `(`: push accumulated result and current sign, then reset.
    /// On `)`: pop sign and previous result, combine.
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<i64> = Vec::new(); // O(n) space for nested parens
        let mut result: i64 = 0;
        let mut sign: i64 = 1;
        let mut num: i64 = 0;

        for c in s.chars() {
            match c {
                '0'..='9' => {
                    num = num * 10 + (c as i64 - '0' as i64); // O(1) accumulate digit
                }
                '+' => {
                    result += sign * num; // flush current number
                    num = 0;
                    sign = 1;
                }
                '-' => {
                    result += sign * num; // flush current number
                    num = 0;
                    sign = -1;
                }
                '(' => {
                    stack.push(result); // O(1) amortized push
                    stack.push(sign);
                    result = 0;
                    sign = 1;
                }
                ')' => {
                    result += sign * num; // flush current number
                    num = 0;
                    let prev_sign = stack.pop().unwrap(); // O(1) pop
                    let prev_result = stack.pop().unwrap();
                    result = prev_result + prev_sign * result;
                }
                _ => {} // skip whitespace
            }
        }
        result += sign * num; // flush trailing number
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::calculate("1 + 1".to_string()), 2);
        assert_eq!(Solution::calculate(" 2-1 + 2 ".to_string()), 3);
        assert_eq!(
            Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()),
            23
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(Solution::calculate("0".to_string()), 0);
        assert_eq!(Solution::calculate("2147483647".to_string()), 2147483647);
        assert_eq!(Solution::calculate("-1".to_string()), -1);
        assert_eq!(Solution::calculate("-(3+2)".to_string()), -5);
        assert_eq!(Solution::calculate("((1+2))".to_string()), 3);
        assert_eq!(Solution::calculate("(7)-(0)+(4)".to_string()), 11);
        assert_eq!(Solution::calculate("1-1+1".to_string()), 1);
        assert_eq!(Solution::calculate("10-5-3".to_string()), 2);
    }
}
