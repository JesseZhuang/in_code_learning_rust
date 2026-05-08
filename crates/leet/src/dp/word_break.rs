use std::collections::{HashSet, VecDeque};

pub struct Solution;

impl Solution {
    /// DP with hash set. O(n^2*k) time, O(n+m*k) space.
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        let mut dp = vec![false; n + 1]; // O(n) space
        dp[0] = true;
        let word_set: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect(); // O(m*k)
        for i in 1..=n { // O(n)
            for j in 0..i { // O(n)
                if dp[j] && word_set.contains(&s[j..i]) { // O(k)
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[n]
    }

    /// BFS approach. O(n^2*k) time, O(n+m*k) space.
    pub fn word_break_bfs(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        let word_set: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();
        queue.push_back(0);
        while let Some(start) = queue.pop_front() { // O(n)
            if start < n && visited[start] {
                continue;
            }
            if start < n {
                visited[start] = true;
            }
            for end in (start + 1)..=n { // O(n)
                if word_set.contains(&s[start..end]) { // O(k)
                    if end == n {
                        return true;
                    }
                    queue.push_back(end);
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_dp() {
        assert!(Solution::word_break("leetcode".into(), to_vec(&["leet", "code"])));
        assert!(Solution::word_break("applepenapple".into(), to_vec(&["apple", "pen"])));
        assert!(!Solution::word_break("catsandog".into(), to_vec(&["cats", "dog", "sand", "and", "cat"])));
        assert!(Solution::word_break("a".into(), to_vec(&["a"])));
        assert!(!Solution::word_break("b".into(), to_vec(&["a"])));
        assert!(Solution::word_break("aaaa".into(), to_vec(&["a", "aa"])));
        assert!(!Solution::word_break("aaaaaab".into(), to_vec(&["a", "aa", "aaa"])));
        assert!(Solution::word_break("catsanddog".into(), to_vec(&["cats", "dog", "sand", "and", "cat"])));
    }

    #[test]
    fn test_bfs() {
        assert!(Solution::word_break_bfs("leetcode".into(), to_vec(&["leet", "code"])));
        assert!(Solution::word_break_bfs("applepenapple".into(), to_vec(&["apple", "pen"])));
        assert!(!Solution::word_break_bfs("catsandog".into(), to_vec(&["cats", "dog", "sand", "and", "cat"])));
        assert!(Solution::word_break_bfs("a".into(), to_vec(&["a"])));
        assert!(!Solution::word_break_bfs("b".into(), to_vec(&["a"])));
        assert!(Solution::word_break_bfs("aaaa".into(), to_vec(&["a", "aa"])));
        assert!(!Solution::word_break_bfs("aaaaaab".into(), to_vec(&["a", "aa", "aaa"])));
        assert!(Solution::word_break_bfs("catsanddog".into(), to_vec(&["cats", "dog", "sand", "and", "cat"])));
    }
}
