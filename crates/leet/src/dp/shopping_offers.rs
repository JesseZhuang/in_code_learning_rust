// lc 638

use std::collections::HashMap;

fn filter_specials(price: &[i32], special: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = price.len();
    special
        .into_iter()
        .filter(|s| {
            let bundle: i32 = (0..n).map(|i| s[i] * price[i]).sum();
            bundle > s[n]
        })
        .collect()
}

impl Solution {
    /// DFS + memoization. 0ms, 2.31mb.
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let n = price.len();
        let special = filter_specials(&price, special);
        let mut memo = HashMap::new();
        Self::dfs(&price, &special, &needs, n, &mut memo)
    }

    fn dfs(
        price: &[i32],
        special: &[Vec<i32>],
        needs: &[i32],
        n: usize,
        memo: &mut HashMap<Vec<i32>, i32>,
    ) -> i32 {
        if let Some(&v) = memo.get(needs) {
            return v;
        }
        let mut res: i32 = (0..n).map(|i| needs[i] * price[i]).sum();
        for offer in special {
            let updated: Vec<i32> = (0..n).map(|i| needs[i] - offer[i]).collect();
            if updated.iter().all(|&u| u >= 0) {
                let cost = offer[n] + Self::dfs(price, special, &updated, n, memo);
                res = res.min(cost);
            }
        }
        memo.insert(needs.to_vec(), res);
        res
    }
}

impl Solution2 {
    /// Backtracking without memoization. 0ms, 2.35mb.
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let n = price.len();
        let special = filter_specials(&price, special);
        Self::dfs(0, &price, &special, &needs, n)
    }

    fn dfs(
        idx: usize,
        price: &[i32],
        special: &[Vec<i32>],
        needs: &[i32],
        n: usize,
    ) -> i32 {
        if idx == special.len() {
            return (0..n).map(|i| needs[i] * price[i]).sum();
        }
        let mut res = Self::dfs(idx + 1, price, special, needs, n);
        let mut updated = needs.to_vec();
        let mut times = 0;
        loop {
            for i in 0..n {
                updated[i] -= special[idx][i];
            }
            if updated.iter().any(|&u| u < 0) {
                break;
            }
            times += 1;
            let branch = special[idx][n] * times + Self::dfs(idx + 1, price, special, &updated, n);
            res = res.min(branch);
        }
        res
    }
}

pub struct Solution;
pub struct Solution2;

#[cfg(test)]
mod tests {
    use crate::dp::shopping_offers::{Solution, Solution2};

    fn assert_both(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>, expected: i32) {
        assert_eq!(
            expected,
            Solution::shopping_offers(price.clone(), special.clone(), needs.clone())
        );
        assert_eq!(
            expected,
            Solution2::shopping_offers(price, special, needs)
        );
    }

    #[test]
    fn example1() {
        assert_both(
            vec![2, 5],
            vec![vec![3, 0, 5], vec![1, 2, 10]],
            vec![3, 2],
            14,
        );
    }

    #[test]
    fn example2() {
        assert_both(
            vec![2, 3, 4],
            vec![vec![1, 1, 0, 4], vec![2, 2, 1, 9]],
            vec![1, 2, 1],
            11,
        );
    }

    #[test]
    fn no_offers() {
        assert_both(vec![1, 2, 3], vec![], vec![3, 2, 1], 10);
    }

    #[test]
    fn no_needs() {
        assert_both(vec![10, 20], vec![vec![1, 1, 5]], vec![0, 0], 0);
    }

    #[test]
    fn single_item() {
        assert_both(vec![5], vec![vec![2, 8]], vec![4], 16);
    }

    #[test]
    fn offer_worse_than_individual() {
        assert_both(vec![1, 1], vec![vec![1, 1, 5]], vec![2, 2], 4);
    }

    #[test]
    fn multiple_same_offer() {
        assert_both(vec![4, 3], vec![vec![2, 1, 5]], vec![4, 2], 10);
    }

    #[test]
    fn offer_exact_match() {
        assert_both(vec![10, 10], vec![vec![1, 1, 1]], vec![1, 1], 1);
    }

    #[test]
    fn large_needs_single_item() {
        assert_both(vec![3], vec![vec![2, 5]], vec![6], 15);
    }

    #[test]
    fn mixed_offers() {
        assert_both(
            vec![2, 3],
            vec![vec![1, 0, 1], vec![0, 1, 2], vec![1, 1, 3]],
            vec![2, 2],
            6,
        );
    }

    #[test]
    fn needs_one_each() {
        assert_both(
            vec![5, 5, 5],
            vec![vec![1, 1, 1, 10]],
            vec![1, 1, 1],
            10,
        );
    }
}
