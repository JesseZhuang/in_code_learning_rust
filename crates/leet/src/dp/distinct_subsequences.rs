/// LeetCode 115 - Distinct Subsequences
/// Given strings s and t, return the number of distinct subsequences of s which equals t.
pub struct Solution;

impl Solution {
    /// 2D DP: dp[i][j] = number of ways to form t[0..j] from s[0..i].
    /// Time O(m*n), Space O(m*n).
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let m = s.len();
        let n = t.len();

        // dp[i][j] = number of distinct subsequences of s[0..i] that equal t[0..j]
        let mut dp = vec![vec![0u64; n + 1]; m + 1];

        // Empty t can be formed from any prefix of s in exactly one way
        for i in 0..=m {
            dp[i][0] = 1;
        }

        // O(m*n) nested iteration over all (i, j) pairs
        for i in 1..=m {
            for j in 1..=n {
                // Carry forward: ways without using s[i-1]
                dp[i][j] = dp[i - 1][j];
                if s[i - 1] == t[j - 1] {
                    // Add ways that use s[i-1] to match t[j-1]
                    dp[i][j] += dp[i - 1][j - 1];
                }
            }
        }

        dp[m][n] as i32
    }

    /// 1D DP (space optimized): traverse j in reverse to avoid overwriting needed values.
    /// Time O(m*n), Space O(n).
    pub fn num_distinct_optimized(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let m = s.len();
        let n = t.len();

        // dp[j] = number of ways to form t[0..j] from the portion of s seen so far
        let mut dp = vec![0u64; n + 1];
        dp[0] = 1; // empty t always matches

        // O(m*n) — for each char in s, update dp in reverse order of j
        for i in 1..=m {
            // Reverse traversal ensures dp[j-1] still holds the value from previous row
            for j in (1..=n).rev() {
                if s[i - 1] == t[j - 1] {
                    dp[j] += dp[j - 1];
                }
            }
        }

        dp[n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::num_distinct("rabbbit".into(), "rabbit".into()), 3);
        assert_eq!(Solution::num_distinct_optimized("rabbbit".into(), "rabbit".into()), 3);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::num_distinct("babgbag".into(), "bag".into()), 5);
        assert_eq!(Solution::num_distinct_optimized("babgbag".into(), "bag".into()), 5);
    }

    #[test]
    fn test_no_match() {
        assert_eq!(Solution::num_distinct("abc".into(), "def".into()), 0);
        assert_eq!(Solution::num_distinct_optimized("abc".into(), "def".into()), 0);
    }

    #[test]
    fn test_empty_t() {
        assert_eq!(Solution::num_distinct("abc".into(), "".into()), 1);
        assert_eq!(Solution::num_distinct_optimized("abc".into(), "".into()), 1);
    }

    #[test]
    fn test_empty_s() {
        assert_eq!(Solution::num_distinct("".into(), "a".into()), 0);
        assert_eq!(Solution::num_distinct_optimized("".into(), "a".into()), 0);
    }

    #[test]
    fn test_both_empty() {
        assert_eq!(Solution::num_distinct("".into(), "".into()), 1);
        assert_eq!(Solution::num_distinct_optimized("".into(), "".into()), 1);
    }

    #[test]
    fn test_equal_strings() {
        assert_eq!(Solution::num_distinct("rabbit".into(), "rabbit".into()), 1);
        assert_eq!(Solution::num_distinct_optimized("rabbit".into(), "rabbit".into()), 1);
    }

    #[test]
    fn test_single_char_repeated() {
        assert_eq!(Solution::num_distinct("aaa".into(), "a".into()), 3);
        assert_eq!(Solution::num_distinct_optimized("aaa".into(), "a".into()), 3);
    }

    #[test]
    fn test_s_shorter_than_t() {
        assert_eq!(Solution::num_distinct("ab".into(), "abc".into()), 0);
        assert_eq!(Solution::num_distinct_optimized("ab".into(), "abc".into()), 0);
    }

    #[test]
    fn test_combinations() {
        // C(5,3) = 10 ways to choose 3 'a's from 5
        assert_eq!(Solution::num_distinct("aaaaa".into(), "aaa".into()), 10);
        assert_eq!(Solution::num_distinct_optimized("aaaaa".into(), "aaa".into()), 10);
    }
}
