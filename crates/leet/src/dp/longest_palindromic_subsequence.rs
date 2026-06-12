pub struct Solution;

impl Solution {
    /// 2D DP approach.
    /// Time: O(n²), Space: O(n²)
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        // dp[i][j] = length of longest palindromic subsequence in s[i..=j]
        let mut dp = vec![vec![0i32; n]; n];

        // Base case: single characters
        for i in 0..n {
            dp[i][i] = 1;
        }

        // Fill by increasing substring length — O(n²) iterations
        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                if s[i] == s[j] {
                    dp[i][j] = dp[i + 1][j - 1] + 2;
                } else {
                    dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
                }
            }
        }

        dp[0][n - 1]
    }

    /// 1D DP (space-optimized) approach.
    /// Time: O(n²), Space: O(n)
    pub fn longest_palindrome_subseq_optimized(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        // dp[j] represents the LPS length for substring ending at j with current start i
        let mut dp = vec![0i32; n];

        // Iterate from right to left — O(n²) total
        for i in (0..n).rev() {
            dp[i] = 1; // base case: single char
            let mut prev = 0; // stores dp[i+1][j-1] from the 2D version
            for j in (i + 1)..n {
                let temp = dp[j]; // save current dp[j] (which is dp[i+1][j])
                if s[i] == s[j] {
                    dp[j] = prev + 2;
                } else {
                    dp[j] = dp[j].max(dp[j - 1]);
                }
                prev = temp;
            }
        }

        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_longest_palindrome_subseq() {
        let cases = vec![
            ("bbbab", 4),
            ("cbbd", 2),
            ("a", 1),
            ("aaaa", 4),
            ("racecar", 7),
            ("abcde", 1),
            ("aa", 2),
            ("ab", 1),
            ("character", 5),
        ];
        for (s, expected) in &cases {
            assert_eq!(
                Solution::longest_palindrome_subseq(s.to_string()),
                *expected,
                "2D failed for {:?}",
                s
            );
            assert_eq!(
                Solution::longest_palindrome_subseq_optimized(s.to_string()),
                *expected,
                "1D failed for {:?}",
                s
            );
        }
    }
}
