/// leet 983

impl Solution {
    /// DP on calendar days. O(lastDay) time, O(lastDay) space.
    /// dp[d] = min cost to cover all travel days up to day d.
    /// Non-travel days: dp[d] = dp[d-1]. Travel days: try each pass type.
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let last = *days.last().unwrap() as usize;
        let mut is_travel = vec![false; last + 1];
        for &d in &days {
            is_travel[d as usize] = true;
        }
        let mut dp = vec![0; last + 1];
        for d in 1..=last {
            if !is_travel[d] {
                dp[d] = dp[d - 1];
            } else {
                dp[d] = dp[d - 1] + costs[0];
                dp[d] = dp[d].min(dp[d.saturating_sub(7)] + costs[1]);
                dp[d] = dp[d].min(dp[d.saturating_sub(30)] + costs[2]);
            }
        }
        dp[last]
    }

    /// DP on travel day indices (bottom-up). O(n) time, O(n) space.
    /// dp[i] = min cost to cover days[i..]. Tabulate from right to left.
    pub fn mincost_tickets2(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let n = days.len();
        let durations = [1, 7, 30];
        let mut dp = vec![0; n + 1]; // dp[n] = 0 (no more days to cover)
        for i in (0..n).rev() {
            dp[i] = i32::MAX;
            for (k, &dur) in durations.iter().enumerate() {
                // Find first index j where days[j] >= days[i] + dur
                let target = days[i] + dur;
                let mut j = i + 1;
                while j < n && days[j] < target {
                    j += 1;
                }
                dp[i] = dp[i].min(costs[k] + dp[j]);
            }
        }
        dp[0]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    fn check(days: Vec<i32>, costs: Vec<i32>, expected: i32) {
        assert_eq!(
            Solution::mincost_tickets(days.clone(), costs.clone()),
            expected,
            "sol1 failed for days={:?}, costs={:?}",
            days,
            costs
        );
        assert_eq!(
            Solution::mincost_tickets2(days.clone(), costs.clone()),
            expected,
            "sol2 failed for days={:?}, costs={:?}",
            days,
            costs
        );
    }

    #[test]
    fn test_cases() {
        check(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15], 11);
        check(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15], 17);
        check(vec![1], vec![2, 7, 15], 2);
        check(vec![1, 2, 3, 4, 5, 6, 7], vec![2, 7, 15], 7);
        check((1..=30).collect(), vec![2, 7, 15], 15);
        check(vec![1, 100, 200], vec![2, 7, 15], 6);
        check(vec![1, 2, 3, 4, 5], vec![1, 10, 100], 5);
        check((1..=10).collect(), vec![5, 5, 5], 5);
        check(vec![1, 365], vec![5, 50, 200], 10);
    }
}
