/// LeetCode 3070 - Count Submatrices With Top-Left Element and Sum Less Than or Equal to K
struct Solution;

impl Solution {
    // 0ms, 9mb
    pub fn count_submatrices(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let (mut res, mut col_sum) = (0i32, vec![0i32; n]);
        for i in 0..m {
            let mut cur = 0;
            for j in 0..n {
                col_sum[j] += grid[i][j];
                cur += col_sum[j];
                if cur <= k {
                    res += 1;
                } else {
                    break;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let grid = vec![vec![7, 6, 3], vec![6, 6, 1]];
        assert_eq!(4, Solution::count_submatrices(grid, 18));
    }

    #[test]
    fn example_2() {
        let grid = vec![vec![7, 2, 9], vec![1, 5, 0], vec![2, 6, 6]];
        assert_eq!(6, Solution::count_submatrices(grid, 20));
    }

    #[test]
    fn edge_single_cell() {
        let grid = vec![vec![5]];
        assert_eq!(1, Solution::count_submatrices(grid, 5));
    }
}
