/// leet 48

impl Solution {
    /// Transpose then reflect. O(n^2) time, O(1) space.
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n { // O(n^2) transpose
            for j in (i + 1)..n {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
        for i in 0..n { // O(n^2) reflect left-right
            for j in 0..n / 2 {
                matrix[i].swap(j, n - 1 - j);
            }
        }
    }

    /// Rotate four cells at a time, layer by layer. O(n^2) time, O(1) space.
    pub fn rotate_four_way(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n / 2 { // O(n/2) layers
            for j in i..n - i - 1 { // O(n) elements per layer
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[n - j - 1][i];
                matrix[n - j - 1][i] = matrix[n - i - 1][n - j - 1];
                matrix[n - i - 1][n - j - 1] = matrix[j][n - i - 1];
                matrix[j][n - i - 1] = tmp;
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_transpose_reflect() {
        let mut m = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
        Solution::rotate(&mut m);
        assert_eq!(m, vec![vec![7,4,1], vec![8,5,2], vec![9,6,3]]);

        let mut m = vec![vec![5,1,9,11], vec![2,4,8,10], vec![13,3,6,7], vec![15,14,12,16]];
        Solution::rotate(&mut m);
        assert_eq!(m, vec![vec![15,13,2,5], vec![14,3,4,1], vec![12,6,8,9], vec![16,7,10,11]]);

        let mut m = vec![vec![1]];
        Solution::rotate(&mut m);
        assert_eq!(m, vec![vec![1]]);

        let mut m = vec![vec![1,2], vec![3,4]];
        Solution::rotate(&mut m);
        assert_eq!(m, vec![vec![3,1], vec![4,2]]);
    }

    #[test]
    fn test_four_way() {
        let mut m = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
        Solution::rotate_four_way(&mut m);
        assert_eq!(m, vec![vec![7,4,1], vec![8,5,2], vec![9,6,3]]);

        let mut m = vec![vec![5,1,9,11], vec![2,4,8,10], vec![13,3,6,7], vec![15,14,12,16]];
        Solution::rotate_four_way(&mut m);
        assert_eq!(m, vec![vec![15,13,2,5], vec![14,3,4,1], vec![12,6,8,9], vec![16,7,10,11]]);

        let mut m = vec![vec![1]];
        Solution::rotate_four_way(&mut m);
        assert_eq!(m, vec![vec![1]]);

        let mut m = vec![vec![1,2], vec![3,4]];
        Solution::rotate_four_way(&mut m);
        assert_eq!(m, vec![vec![3,1], vec![4,2]]);
    }

    #[test]
    fn test_5x5() {
        let mut m = vec![
            vec![1,2,3,4,5], vec![6,7,8,9,10], vec![11,12,13,14,15],
            vec![16,17,18,19,20], vec![21,22,23,24,25],
        ];
        Solution::rotate(&mut m);
        assert_eq!(m, vec![
            vec![21,16,11,6,1], vec![22,17,12,7,2], vec![23,18,13,8,3],
            vec![24,19,14,9,4], vec![25,20,15,10,5],
        ]);
    }
}
