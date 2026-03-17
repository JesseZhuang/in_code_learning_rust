use std::cmp::{max, min};

/// leet 2017, 0 ms, 3.40 mb

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let mut first: i64 = grid[0].iter().map(|&x| x as i64).sum();
        let (mut second, mut res) = (0, i64::MAX);
        for i in 0..grid[0].len() {
            first -= grid[0][i] as i64;
            res = min(res, max(first, second));
            second += grid[1][i - 1] as i64;
        }
        res
    }
}

struct Solution;
