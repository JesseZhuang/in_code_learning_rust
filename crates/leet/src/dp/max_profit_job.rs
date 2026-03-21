// lc 1235

use std::collections::BTreeMap;

impl Solution {
    /// BTreeMap (SortedDict): O(n log n) time, O(n) space.
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs: Vec<(i32, i32, i32)> = start_time
            .into_iter()
            .zip(end_time.into_iter().zip(profit.into_iter()))
            .map(|(s, (e, p))| (s, e, p))
            .collect();
        jobs.sort_by_key(|&(_, e, _)| e);

        let mut tm: BTreeMap<i32, i32> = BTreeMap::new();
        tm.insert(0, 0);

        for (s, e, p) in jobs {
            let base = tm
                .range(..=s)
                .next_back()
                .map(|(_, &v)| v)
                .unwrap_or(0);
            let new_profit = base + p;
            let max_so_far = tm.iter().next_back().map(|(_, &v)| v).unwrap_or(0);
            if new_profit > max_so_far {
                tm.insert(e, new_profit);
            }
        }

        *tm.iter().next_back().unwrap().1
    }
}

impl Solution2 {
    /// DP + binary search on sorted ends: O(n log n) time, O(n) space.
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut jobs: Vec<(i32, i32, i32)> = end_time
            .into_iter()
            .zip(start_time.into_iter().zip(profit.into_iter()))
            .map(|(e, (s, p))| (e, s, p))
            .collect();
        jobs.sort_unstable_by_key(|&(e, _, _)| e);

        let ends: Vec<i32> = jobs.iter().map(|&(e, _, _)| e).collect();
        let mut dp = vec![0; n + 1];
        for i in 1..=n {
            let (_, s, p) = jobs[i - 1];
            let j = ends[..i - 1].partition_point(|&x| x <= s);
            dp[i] = dp[i - 1].max(dp[j] + p);
        }
        dp[n]
    }
}

pub struct Solution;
pub struct Solution2;

#[cfg(test)]
mod tests {
    use crate::dp::max_profit_job::{Solution, Solution2};

    fn assert_both(
        start: Vec<i32>,
        end: Vec<i32>,
        profit: Vec<i32>,
        expected: i32,
    ) {
        assert_eq!(
            expected,
            Solution::job_scheduling(start.clone(), end.clone(), profit.clone())
        );
        assert_eq!(
            expected,
            Solution2::job_scheduling(start, end, profit)
        );
    }

    #[test]
    fn case_1() {
        assert_both(
            vec![1, 2, 3, 3],
            vec![3, 4, 5, 6],
            vec![50, 10, 40, 70],
            120,
        );
    }

    #[test]
    fn case_2() {
        assert_both(
            vec![1, 2, 3, 4, 6],
            vec![3, 5, 10, 6, 9],
            vec![20, 20, 100, 70, 60],
            150,
        );
    }

    #[test]
    fn case_3() {
        assert_both(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4], 6);
    }

    #[test]
    fn case_4() {
        assert_both(vec![1], vec![2], vec![50], 50);
    }

    #[test]
    fn case_5() {
        assert_both(vec![1, 3, 5], vec![2, 4, 6], vec![10, 20, 30], 60);
    }

    #[test]
    fn case_6() {
        assert_both(vec![1, 1, 1], vec![10, 10, 10], vec![5, 6, 4], 6);
    }

    #[test]
    fn case_7() {
        assert_both(vec![1, 2, 3], vec![2, 3, 4], vec![10, 20, 30], 60);
    }

    #[test]
    fn case_8() {
        assert_both(vec![1, 2, 4], vec![3, 5, 6], vec![60, 10, 70], 130);
    }

    #[test]
    fn case_9() {
        assert_both(
            vec![1, 1_000_000_000],
            vec![1_000_000_000, 1_000_000_001],
            vec![100, 200],
            300,
        );
    }
}
