/// leet 97

pub struct Solution;

impl Solution {
    /// 1D DP. O(mn) time, O(n) space.
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let a: Vec<char> = s1.chars().collect();
        let b: Vec<char> = s2.chars().collect();
        let c: Vec<char> = s3.chars().collect();
        let (m, n) = (a.len(), b.len());
        if m + n != c.len() {
            return false;
        }
        // dp[j] = whether s1[0..i] and s2[0..j] can interleave to form s3[0..i+j]
        let mut dp = vec![false; n + 1];
        // O(n) — base case: s1 is empty, check s2 prefix matches s3 prefix
        for j in 0..=n {
            dp[j] = b[..j] == c[..j];
        }
        // O(m*n) — fill row by row
        for i in 1..=m {
            // leftmost column: only using s1[0..i]
            dp[0] = dp[0] && a[i - 1] == c[i - 1];
            for j in 1..=n {
                // O(1) per cell
                dp[j] = (dp[j] && a[i - 1] == c[i + j - 1])
                    || (dp[j - 1] && b[j - 1] == c[i + j - 1]);
            }
        }
        dp[n]
    }

    /// 2D DP. O(mn) time, O(mn) space.
    pub fn is_interleave_2d(s1: String, s2: String, s3: String) -> bool {
        let a: Vec<char> = s1.chars().collect();
        let b: Vec<char> = s2.chars().collect();
        let c: Vec<char> = s3.chars().collect();
        let (m, n) = (a.len(), b.len());
        if m + n != c.len() {
            return false;
        }
        // dp[i][j] = whether s1[0..i] and s2[0..j] can interleave to form s3[0..i+j]
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;
        // O(m) — first column: only s1 contributes
        for i in 1..=m {
            dp[i][0] = dp[i - 1][0] && a[i - 1] == c[i - 1];
        }
        // O(n) — first row: only s2 contributes
        for j in 1..=n {
            dp[0][j] = dp[0][j - 1] && b[j - 1] == c[j - 1];
        }
        // O(m*n) — fill the table
        for i in 1..=m {
            for j in 1..=n {
                // O(1) per cell
                dp[i][j] = (dp[i - 1][j] && a[i - 1] == c[i + j - 1])
                    || (dp[i][j - 1] && b[j - 1] == c[i + j - 1]);
            }
        }
        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn check(s1: &str, s2: &str, s3: &str, expected: bool) {
        assert_eq!(
            Solution::is_interleave(s1.into(), s2.into(), s3.into()),
            expected,
            "1D: is_interleave({:?}, {:?}, {:?})",
            s1, s2, s3
        );
        assert_eq!(
            Solution::is_interleave_2d(s1.into(), s2.into(), s3.into()),
            expected,
            "2D: is_interleave_2d({:?}, {:?}, {:?})",
            s1, s2, s3
        );
    }

    #[test]
    fn test_interleaving_string() {
        check("aabcc", "dbbca", "aadbbcbcac", true);
        check("aabcc", "dbbca", "aadbbbaccc", false);
        check("", "", "", true);
        check("", "", "a", false);
        check("a", "", "a", true);
        check("a", "b", "ab", true);
        check("a", "b", "ba", true);
        check("abc", "def", "adbcef", true);
        check("aaaa", "aaaa", "aaaaaaaa", true);
        check("ab", "cd", "cadb", true);
        check("ab", "cd", "cdba", false);
    }
}
