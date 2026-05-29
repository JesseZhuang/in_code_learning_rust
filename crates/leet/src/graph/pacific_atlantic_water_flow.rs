use std::collections::VecDeque;

/// leet 417
pub struct Solution;

impl Solution {
    /// DFS. O(m*n) time, O(m*n) space.
    /// Start from ocean borders, reverse-flow DFS into cells with height >= current.
    /// Return intersection of cells reachable from both oceans.
    pub fn pacific_atlantic(heights: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        if m == 0 { return vec![]; }
        let n = heights[0].len();

        let mut pacific = vec![vec![false; n]; m];
        let mut atlantic = vec![vec![false; n]; m];

        for i in 0..m {
            Self::dfs(heights, &mut pacific, i, 0, m, n);
            Self::dfs(heights, &mut atlantic, i, n - 1, m, n);
        }
        for j in 0..n {
            Self::dfs(heights, &mut pacific, 0, j, m, n);
            Self::dfs(heights, &mut atlantic, m - 1, j, m, n);
        }

        let mut result = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if pacific[i][j] && atlantic[i][j] {
                    result.push(vec![i as i32, j as i32]);
                }
            }
        }
        result
    }

    fn dfs(
        heights: &[Vec<i32>],
        visited: &mut Vec<Vec<bool>>,
        i: usize,
        j: usize,
        m: usize,
        n: usize,
    ) {
        if visited[i][j] {
            return;
        }
        visited[i][j] = true;
        for (dx, dy) in [(0i32, 1i32), (1, 0), (0, -1), (-1, 0)] {
            let (ni, nj) = (i as i32 + dx, j as i32 + dy);
            if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                let (ni, nj) = (ni as usize, nj as usize);
                if heights[ni][nj] >= heights[i][j] {
                    Self::dfs(heights, visited, ni, nj, m, n);
                }
            }
        }
    }

    /// BFS. O(m*n) time, O(m*n) space.
    pub fn pacific_atlantic_bfs(heights: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        if m == 0 { return vec![]; }
        let n = heights[0].len();

        let mut pacific = vec![vec![false; n]; m];
        let mut atlantic = vec![vec![false; n]; m];

        let mut pq = VecDeque::new();
        let mut aq = VecDeque::new();

        for i in 0..m {
            pacific[i][0] = true;
            pq.push_back((i, 0));
            atlantic[i][n - 1] = true;
            aq.push_back((i, n - 1));
        }
        for j in 0..n {
            if !pacific[0][j] {
                pacific[0][j] = true;
                pq.push_back((0, j));
            }
            if !atlantic[m - 1][j] {
                atlantic[m - 1][j] = true;
                aq.push_back((m - 1, j));
            }
        }

        Self::bfs(heights, &mut pacific, &mut pq, m, n);
        Self::bfs(heights, &mut atlantic, &mut aq, m, n);

        let mut result = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if pacific[i][j] && atlantic[i][j] {
                    result.push(vec![i as i32, j as i32]);
                }
            }
        }
        result
    }

    fn bfs(
        heights: &[Vec<i32>],
        visited: &mut Vec<Vec<bool>>,
        queue: &mut VecDeque<(usize, usize)>,
        m: usize,
        n: usize,
    ) {
        while let Some((i, j)) = queue.pop_front() {
            for (dx, dy) in [(0i32, 1i32), (1, 0), (0, -1), (-1, 0)] {
                let (ni, nj) = (i as i32 + dx, j as i32 + dy);
                if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                    let (ni, nj) = (ni as usize, nj as usize);
                    if !visited[ni][nj] && heights[ni][nj] >= heights[i][j] {
                        visited[ni][nj] = true;
                        queue.push_back((ni, nj));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn sorted(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        v.sort();
        v
    }

    #[test]
    fn example1() {
        let heights = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];
        let expected = vec![
            vec![0, 4], vec![1, 3], vec![1, 4], vec![2, 2],
            vec![3, 0], vec![3, 1], vec![4, 0],
        ];
        assert_eq!(sorted(Solution::pacific_atlantic(&heights)), expected);
        assert_eq!(sorted(Solution::pacific_atlantic_bfs(&heights)), expected);
    }

    #[test]
    fn single_cell() {
        let heights = vec![vec![1]];
        let expected = vec![vec![0, 0]];
        assert_eq!(Solution::pacific_atlantic(&heights), expected);
        assert_eq!(Solution::pacific_atlantic_bfs(&heights), expected);
    }

    #[test]
    fn flat_grid() {
        let heights = vec![vec![1, 1], vec![1, 1]];
        let expected = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]];
        assert_eq!(sorted(Solution::pacific_atlantic(&heights)), expected);
        assert_eq!(sorted(Solution::pacific_atlantic_bfs(&heights)), expected);
    }
}
