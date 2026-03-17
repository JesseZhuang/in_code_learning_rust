/// LeetCode 1727 - Largest Submatrix With Rearrangements
///
/// Solution A: sort heights per row
/// Solution B: counting sort heights per row
struct Solution;

impl Solution {
    // 4ms, 8mb
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut heights = vec![0usize; n];
        let mut best = 0i32;

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 1 {
                    heights[j] += 1;
                } else {
                    heights[j] = 0;
                }
            }
            let mut sorted = heights.clone();
            sorted.sort_unstable_by(|a, b| b.cmp(a));
            for (k, h) in sorted.iter().enumerate() {
                if *h == 0 {
                    break;
                }
                let area = (*h as i32) * ((k + 1) as i32);
                if area > best {
                    best = area;
                }
            }
        }
        best
    }

    // 7ms, 8.62mb
    pub fn largest_submatrix_counting(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut prev: Vec<(usize, usize)> = Vec::new();
        let mut best = 0i32;

        for i in 0..m {
            let mut cur: Vec<(usize, usize)> = Vec::new();
            let mut seen = vec![false; n];

            for &(h, col) in prev.iter() {
                if matrix[i][col] == 1 {
                    cur.push((h + 1, col));
                    seen[col] = true;
                }
            }

            for col in 0..n {
                if !seen[col] && matrix[i][col] == 1 {
                    cur.push((1, col));
                }
            }

            for (idx, (h, _)) in cur.iter().enumerate() {
                let area = (*h as i32) * ((idx + 1) as i32);
                if area > best {
                    best = area;
                }
            }

            prev = cur;
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_both(matrix: Vec<Vec<i32>>, expected: i32) {
        assert_eq!(expected, Solution::largest_submatrix(matrix.clone()));
        assert_eq!(expected, Solution::largest_submatrix_counting(matrix));
    }

    #[test]
    fn example_1() {
        let matrix = vec![vec![0, 0, 1], vec![1, 1, 1], vec![1, 0, 1]];
        assert_both(matrix, 4);
    }

    #[test]
    fn example_2() {
        let matrix = vec![vec![1, 0, 1, 0, 1]];
        assert_both(matrix, 3);
    }

    #[test]
    fn example_3() {
        let matrix = vec![vec![1, 1, 0], vec![1, 0, 1]];
        assert_both(matrix, 2);
    }

    #[test]
    fn edge_single_zero() {
        let matrix = vec![vec![0]];
        assert_both(matrix, 0);
    }

    #[test]
    fn edge_single_one() {
        let matrix = vec![vec![1]];
        assert_both(matrix, 1);
    }

    #[test]
    fn edge_all_zeros() {
        let matrix = vec![vec![0, 0, 0], vec![0, 0, 0]];
        assert_both(matrix, 0);
    }

    #[test]
    fn edge_all_ones() {
        let matrix = vec![vec![1, 1, 1], vec![1, 1, 1]];
        assert_both(matrix, 6);
    }

    #[test]
    fn edge_single_column() {
        let matrix = vec![vec![1], vec![1], vec![0], vec![1]];
        assert_both(matrix, 2);
    }
}
