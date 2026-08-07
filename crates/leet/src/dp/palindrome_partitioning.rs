/// leet 131

impl Solution {
    /// Backtracking + DP palindrome table. O(N*2^N) time, O(N^2) space.
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let n = s.len();
        let bytes = s.as_bytes();
        // dp[i][j] = true if s[i..=j] is a palindrome
        let mut dp = vec![vec![false; n]; n];
        for i in (0..n).rev() { // O(N^2) fill
            for j in i..n {
                dp[i][j] = bytes[i] == bytes[j] && (j - i < 2 || dp[i + 1][j - 1]);
            }
        }
        let mut res = Vec::new();
        let mut path = Vec::new();
        Self::backtrack_dp(&s, 0, &dp, &mut path, &mut res);
        res
    }

    fn backtrack_dp(
        s: &str,
        start: usize,
        dp: &[Vec<bool>],
        path: &mut Vec<String>,
        res: &mut Vec<Vec<String>>,
    ) {
        if start == s.len() {
            res.push(path.clone());
            return;
        }
        for end in start..s.len() { // O(2^N) branches total across recursion tree
            if dp[start][end] {
                path.push(s[start..=end].to_string());
                Self::backtrack_dp(s, end + 1, dp, path, res);
                path.pop();
            }
        }
    }

    /// Backtracking + inline palindrome check. O(N*2^N) time, O(N) space.
    pub fn partition_v2(s: String) -> Vec<Vec<String>> {
        let mut res = Vec::new();
        let mut path = Vec::new();
        Self::backtrack_inline(s.as_bytes(), 0, &mut path, &mut res);
        res
    }

    fn backtrack_inline(
        s: &[u8],
        start: usize,
        path: &mut Vec<String>,
        res: &mut Vec<Vec<String>>,
    ) {
        if start == s.len() {
            res.push(path.clone());
            return;
        }
        for end in start..s.len() { // O(2^N) branches total
            if Self::is_palindrome(s, start, end) { // O(N) check per candidate
                path.push(String::from_utf8_lossy(&s[start..=end]).into_owned());
                Self::backtrack_inline(s, end + 1, path, res);
                path.pop();
            }
        }
    }

    fn is_palindrome(s: &[u8], mut left: usize, mut right: usize) -> bool {
        while left < right {
            if s[left] != s[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    fn sorted(mut v: Vec<Vec<String>>) -> Vec<Vec<String>> {
        v.sort();
        v
    }

    #[test]
    fn test_partition_dp() {
        let res = sorted(Solution::partition("aab".to_string()));
        let expected = sorted(vec![
            vec!["a".into(), "a".into(), "b".into()],
            vec!["aa".into(), "b".into()],
        ]);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_partition_single_char() {
        assert_eq!(
            Solution::partition("a".to_string()),
            vec![vec!["a".to_string()]]
        );
    }

    #[test]
    fn test_partition_all_same() {
        let res = sorted(Solution::partition("aaa".to_string()));
        let expected = sorted(vec![
            vec!["a".into(), "a".into(), "a".into()],
            vec!["a".into(), "aa".into()],
            vec!["aa".into(), "a".into()],
            vec!["aaa".into()],
        ]);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_partition_v2() {
        let res = sorted(Solution::partition_v2("aab".to_string()));
        let expected = sorted(vec![
            vec!["a".into(), "a".into(), "b".into()],
            vec!["aa".into(), "b".into()],
        ]);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_partition_v2_single() {
        assert_eq!(
            Solution::partition_v2("a".to_string()),
            vec![vec!["a".to_string()]]
        );
    }

    #[test]
    fn test_both_match() {
        let input = "abba".to_string();
        let r1 = sorted(Solution::partition(input.clone()));
        let r2 = sorted(Solution::partition_v2(input));
        assert_eq!(r1, r2);
    }
}
