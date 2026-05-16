pub struct Solution;

impl Solution {
    /// Direction-based simulation. O(m*n) time, O(1) space.
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut limits = [n as i32, m as i32 - 1];
        let mut res = Vec::with_capacity(m * n);
        let (mut d, mut r, mut c) = (0usize, 0i32, -1i32);
        while limits[d % 2] > 0 {
            for _ in 0..limits[d % 2] {
                r += dirs[d].0;
                c += dirs[d].1;
                res.push(matrix[r as usize][c as usize]);
            }
            limits[d % 2] -= 1;
            d = (d + 1) % 4;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3x3() {
        let matrix = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
        assert_eq!(Solution::spiral_order(matrix), vec![1,2,3,6,9,8,7,4,5]);
    }

    #[test]
    fn test_3x4() {
        let matrix = vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]];
        assert_eq!(Solution::spiral_order(matrix), vec![1,2,3,4,8,12,11,10,9,5,6,7]);
    }

    #[test]
    fn test_single_row() {
        assert_eq!(Solution::spiral_order(vec![vec![1,2,3,4]]), vec![1,2,3,4]);
    }

    #[test]
    fn test_single_column() {
        assert_eq!(Solution::spiral_order(vec![vec![1],vec![2],vec![3]]), vec![1,2,3]);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::spiral_order(vec![vec![7]]), vec![7]);
    }

    #[test]
    fn test_2x2() {
        assert_eq!(Solution::spiral_order(vec![vec![1,2],vec![3,4]]), vec![1,2,4,3]);
    }

    #[test]
    fn test_4x3() {
        let matrix = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9],vec![10,11,12]];
        assert_eq!(Solution::spiral_order(matrix), vec![1,2,3,6,9,12,11,10,7,4,5,8]);
    }
}
