/// leet 72

impl Solution {
    /// 1D DP. O(mn) time, O(n) space.
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let a: Vec<char> = word1.chars().collect();
        let b: Vec<char> = word2.chars().collect();
        let (m, n) = (a.len(), b.len());
        let mut dp = (0..=n as i32).collect::<Vec<i32>>();
        for i in 1..=m {
            let mut prev = dp[0];
            dp[0] = i as i32;
            for j in 1..=n {
                let tmp = dp[j];
                if a[i - 1] == b[j - 1] {
                    dp[j] = prev;
                } else {
                    dp[j] = 1 + prev.min(dp[j]).min(dp[j - 1]);
                }
                prev = tmp;
            }
        }
        dp[n]
    }

    /// 2D DP. O(mn) time, O(mn) space.
    pub fn min_distance_2d(word1: String, word2: String) -> i32 {
        let a: Vec<char> = word1.chars().collect();
        let b: Vec<char> = word2.chars().collect();
        let (m, n) = (a.len(), b.len());
        let mut dp = vec![vec![0i32; n + 1]; m + 1];
        for i in 0..=m {
            dp[i][0] = i as i32;
        }
        for j in 0..=n {
            dp[0][j] = j as i32;
        }
        for i in 1..=m {
            for j in 1..=n {
                if a[i - 1] == b[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]);
                }
            }
        }
        dp[m][n]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1d() {
        assert_eq!(Solution::min_distance("horse".into(), "ros".into()), 3);
        assert_eq!(Solution::min_distance("intention".into(), "execution".into()), 5);
        assert_eq!(Solution::min_distance("".into(), "".into()), 0);
        assert_eq!(Solution::min_distance("".into(), "abc".into()), 3);
        assert_eq!(Solution::min_distance("abc".into(), "".into()), 3);
        assert_eq!(Solution::min_distance("abc".into(), "abc".into()), 0);
        assert_eq!(Solution::min_distance("a".into(), "b".into()), 1);
        assert_eq!(Solution::min_distance("kitten".into(), "sitting".into()), 3);
        assert_eq!(Solution::min_distance("sunday".into(), "saturday".into()), 3);
    }

    #[test]
    fn test_2d() {
        assert_eq!(Solution::min_distance_2d("horse".into(), "ros".into()), 3);
        assert_eq!(Solution::min_distance_2d("intention".into(), "execution".into()), 5);
        assert_eq!(Solution::min_distance_2d("".into(), "".into()), 0);
        assert_eq!(Solution::min_distance_2d("".into(), "abc".into()), 3);
        assert_eq!(Solution::min_distance_2d("abc".into(), "".into()), 3);
        assert_eq!(Solution::min_distance_2d("abc".into(), "abc".into()), 0);
        assert_eq!(Solution::min_distance_2d("a".into(), "b".into()), 1);
        assert_eq!(Solution::min_distance_2d("kitten".into(), "sitting".into()), 3);
        assert_eq!(Solution::min_distance_2d("sunday".into(), "saturday".into()), 3);
    }
}
