pub struct Solution;

/// Parsed token: either a number or an operator.
#[derive(Clone, Copy)]
enum Token {
    Num(i32),
    Op(u8), // b'+', b'-', b'*'
}

/// Parse expression into alternating [Num, Op, Num, Op, ..., Num].
fn parse(expression: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let bytes = expression.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' | b'-' | b'*' => {
                tokens.push(Token::Op(bytes[i]));
                i += 1;
            }
            _ => {
                let mut n = 0i32;
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    n = n * 10 + (bytes[i] - b'0') as i32;
                    i += 1;
                }
                tokens.push(Token::Num(n));
            }
        }
    }
    tokens
}

fn apply(op: u8, a: i32, b: i32) -> i32 {
    match op {
        b'+' => a + b,
        b'-' => a - b,
        b'*' => a * b,
        _ => unreachable!(),
    }
}

impl Solution {
    /// Tabulation DP: build dp[i][j] = all results for numbers[i..=j].
    /// O(n * 2^n) time, O(2^n) space where n = number of operators (Catalan growth).
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let tokens = parse(&expression);
        // nums[k] = k-th number, ops[k] = operator between nums[k] and nums[k+1]
        let mut nums: Vec<i32> = Vec::new();
        let mut ops: Vec<u8> = Vec::new();
        for t in &tokens {
            match t {
                Token::Num(n) => nums.push(*n),
                Token::Op(o) => ops.push(*o),
            }
        }
        let n = nums.len(); // number of operands
        if n == 0 {
            return vec![];
        }
        // dp[i][j] = all possible results from nums[i..=j]
        let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![]; n]; n];

        // Base case: single numbers
        for i in 0..n {
            dp[i][i] = vec![nums[i]];
        }

        // Fill by increasing length
        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                let mut results = Vec::new();
                // Split at each operator position k (between nums[k] and nums[k+1])
                for k in i..j {
                    let op = ops[k];
                    // Collect left and right into owned vecs to avoid borrow issues
                    let left = dp[i][k].clone();
                    let right = dp[k + 1][j].clone();
                    for &l in &left {
                        for &r in &right {
                            results.push(apply(op, l, r));
                        }
                    }
                }
                dp[i][j] = results;
            }
        }

        dp[0][n - 1].clone()
    }

    /// Memoization (Divide & Conquer) with HashMap cache.
    /// O(n * 2^n) time, O(2^n) space where n = number of operators (Catalan growth).
    pub fn diff_ways_to_compute_memo(expression: String) -> Vec<i32> {
        use std::collections::HashMap;

        let tokens = parse(&expression);
        let mut nums: Vec<i32> = Vec::new();
        let mut ops: Vec<u8> = Vec::new();
        for t in &tokens {
            match t {
                Token::Num(n) => nums.push(*n),
                Token::Op(o) => ops.push(*o),
            }
        }
        let n = nums.len();
        if n == 0 {
            return vec![];
        }

        let mut memo: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

        fn solve(
            i: usize,
            j: usize,
            nums: &[i32],
            ops: &[u8],
            memo: &mut HashMap<(usize, usize), Vec<i32>>,
        ) -> Vec<i32> {
            if let Some(cached) = memo.get(&(i, j)) {
                return cached.clone();
            }
            if i == j {
                let res = vec![nums[i]];
                memo.insert((i, j), res.clone());
                return res;
            }
            let mut results = Vec::new();
            for k in i..j {
                let left = solve(i, k, nums, ops, memo);
                let right = solve(k + 1, j, nums, ops, memo);
                let op = ops[k];
                for &l in &left {
                    for &r in &right {
                        results.push(apply(op, l, r));
                    }
                }
            }
            memo.insert((i, j), results.clone());
            results
        }

        solve(0, n - 1, &nums, &ops, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn cases() -> Vec<(&'static str, Vec<i32>)> {
        vec![
            ("2-1-1", vec![0, 2]),
            ("2*3-4*5", vec![-34, -14, -10, -10, 10]),
            ("3", vec![3]),
            ("11", vec![11]),
            ("2+3", vec![5]),
            ("5-2", vec![3]),
            ("4*3", vec![12]),
            ("1+2+3", vec![6, 6]),
            ("2*3*4", vec![24, 24]),
            ("1+2*3", vec![7, 9]),
            ("10+5", vec![15]),
            ("0+0", vec![0]),
            ("1-2-3", vec![-4, 2]),
        ]
    }

    #[test]
    fn test_tabulation() {
        for (expr, expected) in cases() {
            let mut result = Solution::diff_ways_to_compute(expr.to_string());
            result.sort();
            assert_eq!(result, expected, "failed for expression: {}", expr);
        }
    }

    #[test]
    fn test_memo() {
        for (expr, expected) in cases() {
            let mut result = Solution::diff_ways_to_compute_memo(expr.to_string());
            result.sort();
            assert_eq!(result, expected, "failed for expression: {}", expr);
        }
    }
}
