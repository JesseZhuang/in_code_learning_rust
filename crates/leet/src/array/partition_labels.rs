/// LeetCode 763: Partition Labels
///
/// Given a string s, partition it into as many parts as possible so that
/// each letter appears in at most one part. Return the sizes of the partitions.

// Solution 1: Greedy — O(n) time, O(1) space (26-char array)
pub struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let bytes = s.as_bytes();
        // Track last occurrence of each character — O(n) pass
        let mut last = [0usize; 26];
        for (i, &b) in bytes.iter().enumerate() {
            last[(b - b'a') as usize] = i;
        }
        // Greedily expand partition end — O(n) pass
        let mut result = Vec::new();
        let mut start = 0;
        let mut end = 0;
        for (i, &b) in bytes.iter().enumerate() {
            end = end.max(last[(b - b'a') as usize]);
            if i == end {
                result.push((end - start + 1) as i32);
                start = end + 1;
            }
        }
        result
    }
}

// Solution 2: Merge Intervals — O(n) time, O(1) space (26 intervals max)
pub struct Solution2;

impl Solution2 {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let bytes = s.as_bytes();
        // Build [first, last] interval for each unique character — O(n)
        let mut intervals: [(i32, i32); 26] = [(-1, -1); 26];
        for (i, &b) in bytes.iter().enumerate() {
            let idx = (b - b'a') as usize;
            if intervals[idx].0 == -1 {
                intervals[idx].0 = i as i32;
            }
            intervals[idx].1 = i as i32;
        }
        // Collect only characters that appear, sort by start — O(26 log 26) = O(1)
        let mut present: Vec<(i32, i32)> = intervals.iter().copied().filter(|&(f, _)| f != -1).collect();
        present.sort_unstable();
        // Merge overlapping intervals — O(26) = O(1)
        let mut result = Vec::new();
        let mut cur_start = present[0].0;
        let mut cur_end = present[0].1;
        for &(s, e) in present.iter().skip(1) {
            if s <= cur_end {
                cur_end = cur_end.max(e);
            } else {
                result.push(cur_end - cur_start + 1);
                cur_start = s;
                cur_end = e;
            }
        }
        result.push(cur_end - cur_start + 1);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_cases {
        ($solution:ty) => {
            assert_eq!(<$solution>::partition_labels("ababcbacadefegdehijhklij".to_string()), vec![9, 7, 8]);
            assert_eq!(<$solution>::partition_labels("eccbbbbdec".to_string()), vec![10]);
            assert_eq!(<$solution>::partition_labels("a".to_string()), vec![1]);
            assert_eq!(<$solution>::partition_labels("aaaaa".to_string()), vec![5]);
            assert_eq!(<$solution>::partition_labels("abcd".to_string()), vec![1, 1, 1, 1]);
            assert_eq!(<$solution>::partition_labels("aabb".to_string()), vec![2, 2]);
            assert_eq!(<$solution>::partition_labels("abcbad".to_string()), vec![5, 1]);
            assert_eq!(<$solution>::partition_labels("abcdefghijklmnopqrstuvwxyza".to_string()), vec![27]);
        };
    }

    #[test]
    fn test_solution1() {
        test_cases!(Solution);
    }

    #[test]
    fn test_solution2() {
        test_cases!(Solution2);
    }
}
