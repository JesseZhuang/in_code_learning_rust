// lc 10

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let (n, m, s, p) = (s.len(), p.len(), s.as_bytes(), p.as_bytes());
        let mut dp = vec![vec![false; m + 1]; n + 1];
        dp[n][m] = true;
        for i in (0..n + 1).rev() {
            for j in (0..m).rev() {
                let first_match = i < n && (s[i] == p[j] || p[j] == b'.');
                if j + 1 < m && p[j + 1] == b'*' {
                    dp[i][j] = dp[i][j + 2] || (first_match && dp[i + 1][j]);
                } else { dp[i][j] = first_match && dp[i + 1][j + 1]; }
            }
        }
        dp[0][0]
    }
}

struct Solution;