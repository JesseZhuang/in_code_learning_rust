// lc 2463

impl Solution {
    pub fn minimum_total_distance(mut rob: Vec<i32>, mut fac: Vec<Vec<i32>>) -> i64 {
        rob.sort();
        fac.sort();
        let n = rob.len();
        let mut dp = vec![10i64.pow(12); n + 1]; // scientific notation
        dp[0] = 0;
        for f in fac.iter() {
            for _ in 0..f[1] {
                for (i, &r) in rob.iter().enumerate().rev() {
                    dp[i + 1] = dp[i + 1].min((r - f[0]).abs() as i64 + dp[i]);
                }
            }
        }
        dp[n]
    }
}

struct Solution;