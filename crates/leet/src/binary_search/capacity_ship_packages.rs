/// LeetCode 1011 - Capacity To Ship Packages Within D Days
///
/// Given an array `weights` and integer `days`, return the minimum capacity of
/// the ship so that all packages are shipped within `days` days.

pub struct Solution;

impl Solution {
    /// Binary search on answer + Greedy check.
    /// O(n * log(sum - max)) time, O(1) extra space.
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut lo = *weights.iter().max().unwrap();
        let mut hi: i32 = weights.iter().sum();

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if Self::can_ship(&weights, days, mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }

    /// Greedily count days needed with given capacity.
    fn can_ship(weights: &[i32], days: i32, capacity: i32) -> bool {
        let mut needed = 1;
        let mut current = 0;
        for &w in weights {
            if current + w > capacity {
                needed += 1;
                current = w;
            } else {
                current += w;
            }
        }
        needed <= days
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5),
            15
        );
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
    }

    #[test]
    fn single_package() {
        assert_eq!(Solution::ship_within_days(vec![5], 1), 5);
    }

    #[test]
    fn one_day() {
        assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 4, 5], 1), 15);
    }

    #[test]
    fn equal_weights() {
        assert_eq!(Solution::ship_within_days(vec![3, 3, 3, 3, 3, 3], 3), 6);
    }

    #[test]
    fn heavy_last() {
        assert_eq!(Solution::ship_within_days(vec![1, 1, 1, 500], 2), 500);
    }
}
