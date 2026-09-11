pub struct Solution;

impl Solution {
    /// Bottom-up DP using Catalan recurrence.
    /// O(n^2) time, O(n) space.
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0i32; n + 1];
        dp[0] = 1;
        if n >= 1 {
            dp[1] = 1;
        }
        // O(n^2): for each node count, sum over all possible roots
        for nodes in 2..=n {
            for root in 1..=nodes {
                dp[nodes] += dp[root - 1] * dp[nodes - root];
            }
        }
        dp[n]
    }

    /// Direct Catalan number formula.
    /// O(n) time, O(1) space.
    pub fn num_trees_catalan(n: i32) -> i32 {
        let mut c: i64 = 1;
        // O(n): iterative Catalan computation
        for i in 0..n as i64 {
            c = c * 2 * (2 * i + 1) / (i + 2);
        }
        c as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    const CASES: &[(i32, i32)] = &[
        (0, 1),
        (1, 1),
        (2, 2),
        (3, 5),
        (4, 14),
        (5, 42),
        (19, 1767263190),
    ];

    #[test]
    fn test_num_trees_dp() {
        for &(n, expected) in CASES {
            assert_eq!(Solution::num_trees(n), expected, "dp failed for n={n}");
        }
    }

    #[test]
    fn test_num_trees_catalan() {
        for &(n, expected) in CASES {
            assert_eq!(
                Solution::num_trees_catalan(n),
                expected,
                "catalan failed for n={n}"
            );
        }
    }
}
