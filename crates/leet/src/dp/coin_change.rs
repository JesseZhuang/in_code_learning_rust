/// leet 322

use std::collections::VecDeque;

impl Solution {
    /// Bottom-up DP. O(N*M) time, O(M) space. N: coins.len(), M: amount.
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let m = amount as usize;
        let mut dp = vec![i32::MAX; m + 1]; // dp[i]: min coins for amount i
        dp[0] = 0;
        for &coin in &coins { // O(N)
            let c = coin as usize;
            for i in c..=m { // O(M)
                if dp[i - c] != i32::MAX {
                    dp[i] = dp[i].min(dp[i - c] + 1);
                }
            }
        }
        if dp[m] == i32::MAX { -1 } else { dp[m] }
    }

    /// BFS shortest path. O(N*M) time, O(M) space.
    pub fn coin_change_bfs(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 { return 0; }
        let m = amount as usize;
        let mut visited = vec![false; m + 1];
        let mut q = VecDeque::new();
        q.push_back(amount);
        visited[m] = true;
        let mut count = 0;
        while !q.is_empty() {
            let sz = q.len();
            for _ in 0..sz { // O(level size)
                let cur = q.pop_front().unwrap();
                if cur == 0 { return count; }
                for &coin in &coins { // O(N)
                    let nxt = cur - coin;
                    if nxt >= 0 && !visited[nxt as usize] {
                        q.push_back(nxt);
                        visited[nxt as usize] = true;
                    }
                }
            }
            count += 1;
        }
        -1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_dp() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
        assert_eq!(Solution::coin_change(vec![3], 6), 2);
        assert_eq!(Solution::coin_change(vec![3], 7), -1);
        assert_eq!(Solution::coin_change(vec![1, 5, 10, 25], 30), 2);
        assert_eq!(Solution::coin_change(vec![1, 5, 6], 15), 3);
        assert_eq!(Solution::coin_change(vec![5, 10], 3), -1);
    }

    #[test]
    fn test_bfs() {
        assert_eq!(Solution::coin_change_bfs(vec![1, 2, 5], 11), 3);
        assert_eq!(Solution::coin_change_bfs(vec![2], 3), -1);
        assert_eq!(Solution::coin_change_bfs(vec![1], 0), 0);
        assert_eq!(Solution::coin_change_bfs(vec![3], 6), 2);
        assert_eq!(Solution::coin_change_bfs(vec![3], 7), -1);
        assert_eq!(Solution::coin_change_bfs(vec![1, 5, 10, 25], 30), 2);
        assert_eq!(Solution::coin_change_bfs(vec![1, 5, 6], 15), 3);
        assert_eq!(Solution::coin_change_bfs(vec![5, 10], 3), -1);
    }
}
