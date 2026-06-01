pub struct Solution;

impl Solution {
    /// O(mn) time, O(min(m,n)) space. Rolling 1D DP.
    pub fn unique_paths_dp(m: i32, n: i32) -> i32 {
        let (m, n) = if m > n { (n as usize, m as usize) } else { (m as usize, n as usize) };
        let mut dp = vec![0i32; m + 1];
        dp[1] = 1;
        for _ in 0..n { // O(n)
            for j in 1..=m { // O(m)
                dp[j] += dp[j - 1];
            }
        }
        dp[m]
    }

    /// O(min(m,n)) time, O(1) space. Combinatorics: C(m+n-2, m-1).
    pub fn unique_paths_comb(m: i32, n: i32) -> i32 {
        let (m, n) = if m > n { (n as i64, m as i64) } else { (m as i64, n as i64) };
        let mut res: i64 = 1;
        let mut j: i64 = 1;
        for i in (n..=(m + n - 2)).rev() { // O(min(m,n))
            res = res * i / j;
            j += 1;
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_dp() {
        assert_eq!(28, Solution::unique_paths_dp(3, 7));
        assert_eq!(3, Solution::unique_paths_dp(3, 2));
        assert_eq!(1, Solution::unique_paths_dp(1, 1));
        assert_eq!(1, Solution::unique_paths_dp(1, 100));
        assert_eq!(1, Solution::unique_paths_dp(100, 1));
        assert_eq!(48620, Solution::unique_paths_dp(10, 10));
    }

    #[test]
    fn test_combination() {
        assert_eq!(28, Solution::unique_paths_comb(3, 7));
        assert_eq!(3, Solution::unique_paths_comb(3, 2));
        assert_eq!(1, Solution::unique_paths_comb(1, 1));
        assert_eq!(1, Solution::unique_paths_comb(1, 100));
        assert_eq!(1, Solution::unique_paths_comb(100, 1));
        assert_eq!(48620, Solution::unique_paths_comb(10, 10));
    }
}
