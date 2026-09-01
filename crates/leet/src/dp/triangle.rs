/// leet 120

impl Solution {
    /// Bottom-up DP with O(n) space. O(n^2) time, O(n) space. n: number of rows.
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp = triangle[n - 1].clone(); // O(n) copy last row
        for i in (0..n - 1).rev() { // O(n) rows bottom to top
            for j in 0..=i { // O(i) columns
                dp[j] = triangle[i][j] + dp[j].min(dp[j + 1]);
            }
        }
        dp[0]
    }

    /// In-place bottom-up DP. O(n^2) time, O(1) extra space.
    pub fn minimum_total_in_place(mut triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        for i in (0..n - 1).rev() { // O(n) rows bottom to top
            for j in 0..=i { // O(i) columns
                triangle[i][j] += triangle[i + 1][j].min(triangle[i + 1][j + 1]);
            }
        }
        triangle[0][0]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_dp() {
        assert_eq!(Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]), 11);
        assert_eq!(Solution::minimum_total(vec![vec![-10]]), -10);
        assert_eq!(Solution::minimum_total(vec![vec![1], vec![2, 3]]), 3);
        assert_eq!(Solution::minimum_total(vec![vec![-1], vec![2, 3], vec![1, -1, -3]]), -1);
        assert_eq!(Solution::minimum_total(vec![vec![0], vec![0, 0], vec![0, 0, 0]]), 0);
        assert_eq!(Solution::minimum_total(vec![vec![100], vec![-200, 300], vec![400, -500, 600]]), -600);
        assert_eq!(Solution::minimum_total(vec![vec![1], vec![2, 3], vec![4, 3, 1]]), 5);
    }

    #[test]
    fn test_in_place() {
        assert_eq!(Solution::minimum_total_in_place(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]), 11);
        assert_eq!(Solution::minimum_total_in_place(vec![vec![-10]]), -10);
        assert_eq!(Solution::minimum_total_in_place(vec![vec![1], vec![2, 3]]), 3);
        assert_eq!(Solution::minimum_total_in_place(vec![vec![-1], vec![2, 3], vec![1, -1, -3]]), -1);
        assert_eq!(Solution::minimum_total_in_place(vec![vec![0], vec![0, 0], vec![0, 0, 0]]), 0);
        assert_eq!(Solution::minimum_total_in_place(vec![vec![100], vec![-200, 300], vec![400, -500, 600]]), -600);
        assert_eq!(Solution::minimum_total_in_place(vec![vec![1], vec![2, 3], vec![4, 3, 1]]), 5);
    }
}
