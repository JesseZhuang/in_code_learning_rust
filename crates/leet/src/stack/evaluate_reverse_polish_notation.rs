/// LeetCode 150 - Evaluate Reverse Polish Notation
///
/// Time: O(n), Space: O(n)
pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        for token in &tokens {
            match token.as_str() {
                "+" | "-" | "*" | "/" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let result = match token.as_str() {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b, // Rust i32 division truncates toward zero
                        _ => unreachable!(),
                    };
                    stack.push(result);
                }
                num => stack.push(num.parse::<i32>().unwrap()),
            }
        }
        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example1() {
        // ["2","1","+","3","*"] => ((2+1)*3) = 9
        assert_eq!(Solution::eval_rpn(to_vec(&["2", "1", "+", "3", "*"])), 9);
    }

    #[test]
    fn example2() {
        // ["4","13","5","/","+"] => (4+(13/5)) = 6
        assert_eq!(Solution::eval_rpn(to_vec(&["4", "13", "5", "/", "+"])), 6);
    }

    #[test]
    fn example3() {
        // ["10","6","9","3","+","-11","*","/","*","17","+","5","+"] => 22
        assert_eq!(
            Solution::eval_rpn(to_vec(&[
                "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
            ])),
            22
        );
    }

    #[test]
    fn single_number() {
        assert_eq!(Solution::eval_rpn(to_vec(&["42"])), 42);
    }

    #[test]
    fn negative_result() {
        // ["3", "5", "-"] => 3 - 5 = -2
        assert_eq!(Solution::eval_rpn(to_vec(&["3", "5", "-"])), -2);
    }

    #[test]
    fn division_truncates_toward_zero() {
        // ["7", "-3", "/"] => 7 / -3 = -2 (truncated toward zero)
        assert_eq!(Solution::eval_rpn(to_vec(&["7", "-3", "/"])), -2);
    }
}
