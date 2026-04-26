/// leet 39

impl Solution {
    /// Backtracking with sort + pruning. O(n^(t/m)) time, O(t/m) recursion depth.
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut res = Vec::new();
        Self::backtrack(&candidates, target, 0, &mut vec![], &mut res);
        res
    }

    fn backtrack(c: &[i32], remaining: i32, start: usize, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if remaining == 0 {
            res.push(path.clone());
            return;
        }
        for i in start..c.len() { // O(n) branches per level
            if c[i] > remaining { break; } // prune: sorted
            path.push(c[i]);
            Self::backtrack(c, remaining - c[i], i, path, res); // same i: allow reuse
            path.pop();
        }
    }

    /// Iterative DP. O(n*t*k) time, O(t*C) space.
    pub fn combination_sum_dp(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let target = target as usize;
        let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![]; target + 1];
        dp[0] = vec![vec![]];
        for c in &candidates { // O(n)
            let c = *c;
            for s in c as usize..=target { // O(t)
                let prev = dp[s - c as usize].clone();
                for mut combo in prev {
                    combo.push(c);
                    dp[s].push(combo);
                }
            }
        }
        dp[target].clone()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    fn sorted(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for c in &mut v { c.sort(); }
        v.sort();
        v
    }

    #[test]
    fn test_backtrack() {
        assert_eq!(sorted(vec![vec![2,2,3],vec![7]]), sorted(Solution::combination_sum(vec![2,3,6,7], 7)));
        assert_eq!(sorted(vec![vec![2,2,2,2],vec![2,3,3],vec![3,5]]), sorted(Solution::combination_sum(vec![2,3,5], 8)));
        assert_eq!(sorted(vec![] as Vec<Vec<i32>>), sorted(Solution::combination_sum(vec![2], 1)));
        assert_eq!(sorted(vec![vec![1]]), sorted(Solution::combination_sum(vec![1], 1)));
        assert_eq!(sorted(vec![vec![1,1]]), sorted(Solution::combination_sum(vec![1], 2)));
        assert_eq!(sorted(vec![vec![1,1,1,1],vec![1,1,2],vec![2,2]]), sorted(Solution::combination_sum(vec![1,2], 4)));
    }

    #[test]
    fn test_dp() {
        assert_eq!(sorted(vec![vec![2,2,3],vec![7]]), sorted(Solution::combination_sum_dp(vec![2,3,6,7], 7)));
        assert_eq!(sorted(vec![vec![2,2,2,2],vec![2,3,3],vec![3,5]]), sorted(Solution::combination_sum_dp(vec![2,3,5], 8)));
        assert_eq!(sorted(vec![] as Vec<Vec<i32>>), sorted(Solution::combination_sum_dp(vec![2], 1)));
        assert_eq!(sorted(vec![vec![1]]), sorted(Solution::combination_sum_dp(vec![1], 1)));
        assert_eq!(sorted(vec![vec![1,1]]), sorted(Solution::combination_sum_dp(vec![1], 2)));
        assert_eq!(sorted(vec![vec![1,1,1,1],vec![1,1,2],vec![2,2]]), sorted(Solution::combination_sum_dp(vec![1,2], 4)));
    }
}
