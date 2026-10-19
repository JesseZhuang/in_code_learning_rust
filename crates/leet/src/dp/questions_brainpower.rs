pub struct Solution;

impl Solution {
    /// O(n) time and O(n) space.
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let n = questions.len();
        let mut dp = vec![0i64; n + 1];

        for i in (0..n).rev() {
            let next = (i + questions[i][1] as usize + 1).min(n);
            dp[i] = dp[i + 1].max(questions[i][0] as i64 + dp[next]);
        }

        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn official_examples() {
        assert_eq!(
            Solution::most_points(vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]]),
            5
        );
        assert_eq!(
            Solution::most_points(vec![
                vec![1, 1],
                vec![2, 2],
                vec![3, 3],
                vec![4, 4],
                vec![5, 5],
            ]),
            7
        );
    }

    #[test]
    fn one_question() {
        assert_eq!(Solution::most_points(vec![vec![42, 5]]), 42);
    }

    #[test]
    fn solve_or_skip() {
        assert_eq!(
            Solution::most_points(vec![vec![10, 1], vec![1, 1], vec![7, 1]]),
            17
        );
        assert_eq!(
            Solution::most_points(vec![vec![2, 2], vec![4, 1], vec![5, 1]]),
            5
        );
    }

    #[test]
    fn jump_past_end() {
        assert_eq!(
            Solution::most_points(vec![vec![10, 100_000], vec![4, 1]]),
            10
        );
    }

    #[test]
    fn maximum_length_and_points() {
        assert_eq!(
            Solution::most_points(vec![vec![100_000, 1]; 100_000]),
            5_000_000_000
        );
    }
}
