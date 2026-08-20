use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    /// Min-heap approach. O(n log n) time, O(n) space.
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_unstable(); // O(n log n)
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        for iv in &intervals { // O(n)
            if let Some(&Reverse(earliest_end)) = heap.peek() {
                if earliest_end <= iv[0] {
                    heap.pop(); // O(log n)
                }
            }
            heap.push(Reverse(iv[1])); // O(log n)
        }

        heap.len() as i32
    }
}

pub struct Solution2;

impl Solution2 {
    /// Sweep line: sort starts and ends separately. O(n log n) time, O(n) space.
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let n = intervals.len();
        let mut starts: Vec<i32> = intervals.iter().map(|iv| iv[0]).collect();
        let mut ends: Vec<i32> = intervals.iter().map(|iv| iv[1]).collect();
        starts.sort_unstable(); // O(n log n)
        ends.sort_unstable(); // O(n log n)

        let mut rooms = 0;
        let mut end_ptr = 0;

        for i in 0..n { // O(n)
            if starts[i] < ends[end_ptr] {
                rooms += 1;
            } else {
                end_ptr += 1;
            }
        }

        rooms
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
        assert_eq!(Solution::min_meeting_rooms(intervals.clone()), 2);
        assert_eq!(Solution2::min_meeting_rooms(intervals), 2);
    }

    #[test]
    fn example2() {
        let intervals = vec![vec![7, 10], vec![2, 4]];
        assert_eq!(Solution::min_meeting_rooms(intervals.clone()), 1);
        assert_eq!(Solution2::min_meeting_rooms(intervals), 1);
    }

    #[test]
    fn all_overlap() {
        let intervals = vec![vec![1, 10], vec![2, 7], vec![3, 19]];
        assert_eq!(Solution::min_meeting_rooms(intervals.clone()), 3);
        assert_eq!(Solution2::min_meeting_rooms(intervals), 3);
    }

    #[test]
    fn back_to_back() {
        let intervals = vec![vec![1, 5], vec![5, 10], vec![10, 15]];
        assert_eq!(Solution::min_meeting_rooms(intervals.clone()), 1);
        assert_eq!(Solution2::min_meeting_rooms(intervals), 1);
    }

    #[test]
    fn all_same_time() {
        let intervals = vec![vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2]];
        assert_eq!(Solution::min_meeting_rooms(intervals.clone()), 4);
        assert_eq!(Solution2::min_meeting_rooms(intervals), 4);
    }

    #[test]
    fn single() {
        let intervals = vec![vec![1, 5]];
        assert_eq!(Solution::min_meeting_rooms(intervals.clone()), 1);
        assert_eq!(Solution2::min_meeting_rooms(intervals), 1);
    }

    #[test]
    fn no_overlap() {
        let intervals = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        assert_eq!(Solution::min_meeting_rooms(intervals.clone()), 1);
        assert_eq!(Solution2::min_meeting_rooms(intervals), 1);
    }
}
