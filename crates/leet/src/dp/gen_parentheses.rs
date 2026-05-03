/// leet 22

impl Solution {
    /// Backtracking approach.
    /// Time: O(4^n / sqrt(n)) — Catalan number of valid sequences.
    /// Space: O(4^n / sqrt(n)) — storing all results + O(n) recursion stack.
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        Self::backtrack(n as usize, 0, 0, &mut current, &mut result);
        result
    }

    fn backtrack(
        n: usize,
        open: usize,
        close: usize,
        current: &mut String,
        result: &mut Vec<String>,
    ) {
        if current.len() == 2 * n {
            // Base case: valid sequence of length 2n
            result.push(current.clone());
            return;
        }
        if open < n {
            current.push('(');
            Self::backtrack(n, open + 1, close, current, result);
            current.pop();
        }
        if close < open {
            current.push(')');
            Self::backtrack(n, open, close + 1, current, result);
            current.pop();
        }
    }

    /// DP decomposition: f(k) = "(" + f(i) + ")" + f(k-1-i) for i in [0, k).
    /// Time: O(4^n / sqrt(n)) — generates all Catalan(n) strings.
    /// Space: O(4^n / sqrt(n)) — stores all results for each sub-problem.
    pub fn generate_parenthesis_dp(n: i32) -> Vec<String> {
        let n = n as usize;
        // dp[i] holds all valid parentheses strings of i pairs
        let mut dp: Vec<Vec<String>> = vec![Vec::new(); n + 1];
        dp[0].push(String::new()); // base case: empty string for 0 pairs

        for k in 1..=n {
            let mut results = Vec::new();
            for i in 0..k {
                // Split: "(" + f(i) + ")" + f(k-1-i)
                let left = dp[i].clone();
                let right = dp[k - 1 - i].clone();
                for l in &left {
                    for r in &right {
                        results.push(format!("({}){}", l, r));
                    }
                }
            }
            dp[k] = results;
        }

        dp[n].clone()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_backtracking() {
        // n = 1
        let mut res = Solution::generate_parenthesis(1);
        res.sort();
        assert_eq!(res, vec!["()"]);

        // n = 2
        let mut res = Solution::generate_parenthesis(2);
        res.sort();
        assert_eq!(res, vec!["(())", "()()"]);

        // n = 3
        let mut res = Solution::generate_parenthesis(3);
        res.sort();
        assert_eq!(res, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);

        // n = 4: Catalan(4) = 14
        let res = Solution::generate_parenthesis(4);
        assert_eq!(res.len(), 14);
    }

    #[test]
    fn test_dp() {
        // n = 1
        let mut res = Solution::generate_parenthesis_dp(1);
        res.sort();
        assert_eq!(res, vec!["()"]);

        // n = 2
        let mut res = Solution::generate_parenthesis_dp(2);
        res.sort();
        assert_eq!(res, vec!["(())", "()()"]);

        // n = 3
        let mut res = Solution::generate_parenthesis_dp(3);
        res.sort();
        assert_eq!(res, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);

        // n = 4: Catalan(4) = 14
        let res = Solution::generate_parenthesis_dp(4);
        assert_eq!(res.len(), 14);
    }
}
