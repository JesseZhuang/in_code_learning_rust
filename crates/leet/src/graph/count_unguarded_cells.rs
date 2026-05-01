/// LeetCode 2257

pub struct Solution;

impl Solution {
    /// Simulation. O(mn + g(m+n)) time, O(mn) space.
    pub fn count_unguarded(m: i32, n: i32, guards: &[Vec<i32>], walls: &[Vec<i32>]) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut grid = vec![vec![0u8; n]; m];
        const GUARD: u8 = 2;
        const WALL: u8 = 3;
        const GUARDED: u8 = 1;
        for g in guards {
            grid[g[0] as usize][g[1] as usize] = GUARD;
        }
        for w in walls {
            grid[w[0] as usize][w[1] as usize] = WALL;
        }
        let dirs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for g in guards {
            for &(dr, dc) in &dirs {
                let (mut nr, mut nc) = (g[0] + dr, g[1] + dc);
                while nr >= 0 && nc >= 0 && (nr as usize) < m && (nc as usize) < n
                    && grid[nr as usize][nc as usize] != GUARD
                    && grid[nr as usize][nc as usize] != WALL
                {
                    grid[nr as usize][nc as usize] = GUARDED;
                    nr += dr;
                    nc += dc;
                }
            }
        }
        grid.iter().flatten().filter(|&&c| c == 0).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let guards = vec![vec![0, 0], vec![1, 1], vec![2, 3]];
        let walls = vec![vec![0, 1], vec![2, 2], vec![1, 4]];
        assert_eq!(Solution::count_unguarded(4, 6, &guards, &walls), 7);
    }

    #[test]
    fn example2() {
        let guards = vec![vec![1, 1]];
        let walls = vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]];
        assert_eq!(Solution::count_unguarded(3, 3, &guards, &walls), 4);
    }
}
