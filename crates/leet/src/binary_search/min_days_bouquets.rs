/// LeetCode 1482 - Minimum Number of Days to Make m Bouquets
///
/// Given an array `bloom_day`, integers `m` (bouquets needed) and `k` (adjacent
/// flowers per bouquet), return the minimum number of days to wait so you can
/// make `m` bouquets. Return -1 if impossible.

pub struct Solution;

impl Solution {
    /// Binary search on answer + Greedy check.
    /// O(n * log(max_day)) time, O(1) extra space.
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        let n = bloom_day.len();
        // Impossible if we need more flowers than available
        if (m as i64) * (k as i64) > n as i64 {
            return -1;
        }

        let mut lo = *bloom_day.iter().min().unwrap();
        let mut hi = *bloom_day.iter().max().unwrap();

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if Self::can_make(&bloom_day, m, k, mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }

    /// Greedily count bouquets formable by day `days`.
    fn can_make(bloom_day: &[i32], m: i32, k: i32, days: i32) -> bool {
        let mut bouquets = 0;
        let mut consecutive = 0;
        for &d in bloom_day {
            if d <= days {
                consecutive += 1;
                if consecutive == k {
                    bouquets += 1;
                    consecutive = 0;
                }
            } else {
                consecutive = 0;
            }
        }
        bouquets >= m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 1), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 2), -1);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3), 12);
    }

    #[test]
    fn all_same() {
        assert_eq!(Solution::min_days(vec![5, 5, 5, 5], 2, 2), 5);
    }

    #[test]
    fn single() {
        assert_eq!(Solution::min_days(vec![1], 1, 1), 1);
    }

    #[test]
    fn impossible() {
        assert_eq!(Solution::min_days(vec![1, 2, 3], 2, 2), -1);
    }

    #[test]
    fn max_day() {
        assert_eq!(
            Solution::min_days(vec![1000000000, 1000000000], 1, 2),
            1000000000
        );
    }

    #[test]
    fn already_bloomed() {
        assert_eq!(Solution::min_days(vec![1, 1, 1, 1], 2, 2), 1);
    }
}
