/// leet 465, lint 707

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// Subset sum DP approach. O(n * 2^n) time, O(2^n) space.
    pub fn min_transfers(transactions: Vec<Vec<i32>>) -> i32 {
        let mut bal: HashMap<i32, i32> = HashMap::new();
        for t in &transactions {
            *bal.entry(t[0]).or_insert(0) -= t[2];
            *bal.entry(t[1]).or_insert(0) += t[2];
        }
        let non_zero: Vec<i32> = bal.values().filter(|&&v| v != 0).copied().collect();
        let n = non_zero.len();
        let mut f = vec![i32::MAX; 1 << n];
        f[0] = 0;
        for i in 1..(1 << n) {
            let mut total: i32 = 0;
            for j in 0..n {
                if (i >> j) & 1 == 1 {
                    total += non_zero[j];
                }
            }
            if total == 0 {
                f[i] = (i as u32).count_ones() as i32 - 1;
                let mut j = (i - 1) & i;
                while j > 0 {
                    if f[j] < i32::MAX && f[i ^ j] < i32::MAX {
                        f[i] = f[i].min(f[j] + f[i ^ j]);
                    }
                    j = (j - 1) & i;
                }
            }
        }
        f[(1 << n) - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_transfers() {
        assert_eq!(
            2,
            Solution::min_transfers(vec![vec![0, 1, 10], vec![2, 0, 5]])
        );
        assert_eq!(
            1,
            Solution::min_transfers(vec![
                vec![0, 1, 10],
                vec![1, 0, 1],
                vec![1, 2, 5],
                vec![2, 0, 5]
            ])
        );
        assert_eq!(
            2,
            Solution::min_transfers(vec![
                vec![0, 1, 10],
                vec![1, 2, 15],
                vec![2, 3, 10]
            ])
        );
    }
}
