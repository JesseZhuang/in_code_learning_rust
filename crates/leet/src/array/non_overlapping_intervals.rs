pub struct Solution;

impl Solution {
    /// Sort by end time, greedily count non-overlapping intervals.
    /// Remove count = total - non-overlapping count.
    /// O(n log n) time, O(log n) space (sort in-place)
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }
        intervals.sort_unstable_by_key(|v| v[1]); // O(n log n) time, O(log n) space
        let mut end = intervals[0][1];
        let mut non_overlap = 1;
        for i in 1..intervals.len() {
            if intervals[i][0] >= end {
                // no overlap, extend chain
                non_overlap += 1;
                end = intervals[i][1];
            }
        }
        (intervals.len() as i32) - non_overlap
    }

    /// Sort by start time, when overlap keep interval with smaller end.
    /// O(n log n) time, O(log n) space (sort in-place)
    pub fn erase_overlap_intervals_v2(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }
        intervals.sort_unstable_by_key(|v| v[0]); // O(n log n) time, O(log n) space
        let mut end = intervals[0][1];
        let mut removals = 0;
        for i in 1..intervals.len() {
            if intervals[i][0] < end {
                // overlap: remove the one with larger end
                removals += 1;
                end = end.min(intervals[i][1]); // keep smaller end
            } else {
                end = intervals[i][1];
            }
        }
        removals
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn cases() -> Vec<(Vec<Vec<i32>>, i32)> {
        vec![
            (vec![vec![1,2],vec![2,3],vec![3,4],vec![1,3]], 1),
            (vec![vec![1,2],vec![1,2],vec![1,2]], 2),
            (vec![vec![1,2],vec![2,3]], 0),
            (vec![vec![1,5]], 0),
            (vec![vec![1,4],vec![2,5],vec![3,6]], 2),
            (vec![vec![1,10],vec![2,3],vec![4,5],vec![6,7]], 1),
            (vec![vec![-5,-3],vec![-4,-1],vec![0,2]], 1),
        ]
    }

    #[test]
    fn test_erase_overlap_intervals() {
        for (intervals, expected) in cases() {
            assert_eq!(Solution::erase_overlap_intervals(intervals), expected);
        }
    }

    #[test]
    fn test_erase_overlap_intervals_v2() {
        for (intervals, expected) in cases() {
            assert_eq!(Solution::erase_overlap_intervals_v2(intervals), expected);
        }
    }
}
