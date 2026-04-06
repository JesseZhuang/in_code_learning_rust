// lc 874

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let dx = [0, 1, 0, -1]; // N, E, S, W
        let dy = [1, 0, -1, 0];
        let obs: HashSet<(i32, i32)> = obstacles.iter().map(|o| (o[0], o[1])).collect();
        let (mut x, mut y, mut d) = (0i32, 0i32, 0usize);
        let mut res = 0;
        for &c in &commands {
            if c == -2 {
                d = (d + 3) % 4;
            } else if c == -1 {
                d = (d + 1) % 4;
            } else {
                for _ in 0..c {
                    let (nx, ny) = (x + dx[d], y + dy[d]);
                    if obs.contains(&(nx, ny)) {
                        break;
                    }
                    x = nx;
                    y = ny;
                }
                res = res.max(x * x + y * y);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::robot_sim(vec![4, -1, 3], vec![]), 25);
        assert_eq!(Solution::robot_sim(vec![4, -1, 4, -2, 4], vec![vec![2, 4]]), 65);
        assert_eq!(Solution::robot_sim(vec![6, -1, -1, 6], vec![vec![0, 0]]), 36);
        assert_eq!(Solution::robot_sim(vec![-2, -1, -2, -1], vec![]), 0);
        assert_eq!(Solution::robot_sim(vec![1], vec![]), 1);
        assert_eq!(Solution::robot_sim(vec![4], vec![vec![0, 1]]), 0);
        assert_eq!(Solution::robot_sim(vec![4, -1, 3, -1, 2, -1, 1], vec![]), 25);
        assert_eq!(Solution::robot_sim(vec![-2, 3], vec![]), 9);
        assert_eq!(Solution::robot_sim(vec![9, -1, 9], vec![vec![0, 3], vec![3, 2]]), 8);
    }
}
