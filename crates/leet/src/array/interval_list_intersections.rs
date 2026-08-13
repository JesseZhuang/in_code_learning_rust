pub struct Solution;

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let (mut i, mut j) = (0, 0);
        while i < first_list.len() && j < second_list.len() {
            let lo = first_list[i][0].max(second_list[j][0]);
            let hi = first_list[i][1].min(second_list[j][1]);
            if lo <= hi {
                res.push(vec![lo, hi]);
            }
            if first_list[i][1] < second_list[j][1] {
                i += 1;
            } else {
                j += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let first = vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]];
        let second = vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]];
        let expected = vec![
            vec![1, 2],
            vec![5, 5],
            vec![8, 10],
            vec![15, 23],
            vec![24, 24],
            vec![25, 25],
        ];
        assert_eq!(Solution::interval_intersection(first, second), expected);
    }

    #[test]
    fn test_example2_empty_first() {
        let first: Vec<Vec<i32>> = vec![];
        let second = vec![vec![1, 3], vec![5, 9]];
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::interval_intersection(first, second), expected);
    }

    #[test]
    fn test_empty_second() {
        let first = vec![vec![1, 3], vec![5, 9]];
        let second: Vec<Vec<i32>> = vec![];
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::interval_intersection(first, second), expected);
    }

    #[test]
    fn test_no_intersection() {
        let first = vec![vec![1, 2], vec![5, 6]];
        let second = vec![vec![3, 4], vec![7, 8]];
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::interval_intersection(first, second), expected);
    }

    #[test]
    fn test_full_overlap() {
        let first = vec![vec![1, 10]];
        let second = vec![vec![1, 10]];
        let expected = vec![vec![1, 10]];
        assert_eq!(Solution::interval_intersection(first, second), expected);
    }

    #[test]
    fn test_touching_endpoints() {
        let first = vec![vec![1, 3]];
        let second = vec![vec![3, 5]];
        let expected = vec![vec![3, 3]];
        assert_eq!(Solution::interval_intersection(first, second), expected);
    }
}
