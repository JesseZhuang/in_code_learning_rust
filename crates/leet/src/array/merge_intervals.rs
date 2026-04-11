/// LeetCode 56 - Merge Intervals
/// Time: O(n log n) — dominated by the sort
/// Space: O(n) — for the result vector
struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|v| v[0]);
        let mut res: Vec<Vec<i32>> = Vec::new();
        for iv in intervals {
            if let Some(last) = res.last_mut() {
                if iv[0] <= last[1] {
                    last[1] = last[1].max(iv[1]);
                    continue;
                }
            }
            res.push(iv);
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
            vec![vec![1, 6], vec![8, 10], vec![15, 18]],
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![vec![1, 5]],
            Solution::merge(vec![vec![1, 4], vec![4, 5]])
        );
    }

    #[test]
    fn single() {
        assert_eq!(vec![vec![1, 1]], Solution::merge(vec![vec![1, 1]]));
    }

    #[test]
    fn no_overlap() {
        assert_eq!(
            vec![vec![1, 2], vec![4, 5], vec![7, 8]],
            Solution::merge(vec![vec![1, 2], vec![4, 5], vec![7, 8]])
        );
    }

    #[test]
    fn all_overlap() {
        assert_eq!(
            vec![vec![1, 10]],
            Solution::merge(vec![vec![1, 10], vec![2, 3], vec![4, 5], vec![6, 7]])
        );
    }

    #[test]
    fn unsorted() {
        assert_eq!(
            vec![vec![1, 4], vec![5, 6]],
            Solution::merge(vec![vec![3, 4], vec![1, 2], vec![5, 6], vec![2, 3]])
        );
    }

    #[test]
    fn nested() {
        assert_eq!(
            vec![vec![1, 5]],
            Solution::merge(vec![vec![1, 5], vec![2, 3]])
        );
    }

    #[test]
    fn touching_edges() {
        assert_eq!(
            vec![vec![1, 4]],
            Solution::merge(vec![vec![1, 2], vec![2, 3], vec![3, 4]])
        );
    }
}
