/// LeetCode 57 - Insert Interval
/// Time: O(n) — single pass through intervals
/// Space: O(n) — for the result vector
pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut new = new_interval;
        let mut i = 0;
        let n = intervals.len();
        while i < n && intervals[i][1] < new[0] {
            res.push(intervals[i].clone());
            i += 1;
        }
        while i < n && intervals[i][0] <= new[1] {
            new[0] = new[0].min(intervals[i][0]);
            new[1] = new[1].max(intervals[i][1]);
            i += 1;
        }
        res.push(new);
        while i < n {
            res.push(intervals[i].clone());
            i += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![vec![1, 5], vec![6, 9]],
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![vec![1, 2], vec![3, 10], vec![12, 16]],
            Solution::insert(
                vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]],
                vec![4, 8]
            )
        );
    }

    #[test]
    fn empty_intervals() {
        assert_eq!(
            vec![vec![5, 7]],
            Solution::insert(vec![], vec![5, 7])
        );
    }

    #[test]
    fn new_inside_existing() {
        assert_eq!(
            vec![vec![1, 5]],
            Solution::insert(vec![vec![1, 5]], vec![2, 3])
        );
    }

    #[test]
    fn new_after_all() {
        assert_eq!(
            vec![vec![1, 5], vec![6, 8]],
            Solution::insert(vec![vec![1, 5]], vec![6, 8])
        );
    }

    #[test]
    fn new_before_all() {
        assert_eq!(
            vec![vec![0, 0], vec![1, 5]],
            Solution::insert(vec![vec![1, 5]], vec![0, 0])
        );
    }

    #[test]
    fn new_covers_all() {
        assert_eq!(
            vec![vec![0, 7]],
            Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6]], vec![0, 7])
        );
    }

    #[test]
    fn new_between_gaps() {
        assert_eq!(
            vec![vec![1, 2], vec![3, 4], vec![5, 6]],
            Solution::insert(vec![vec![1, 2], vec![5, 6]], vec![3, 4])
        );
    }
}
