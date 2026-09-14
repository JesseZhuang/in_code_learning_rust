use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// Sliding window + HashMap approach.
    /// Maintain a window [left, right] with at most 2 distinct fruit types.
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut count: HashMap<i32, i32> = HashMap::new();
        let mut left = 0;
        let mut ans = 0;

        for right in 0..fruits.len() {
            *count.entry(fruits[right]).or_insert(0) += 1;

            while count.len() > 2 {
                let lf = fruits[left];
                let c = count.get_mut(&lf).unwrap();
                *c -= 1;
                if *c == 0 {
                    count.remove(&lf);
                }
                left += 1;
            }

            ans = ans.max(right - left + 1);
        }

        ans as i32
    }

    /// Track-last-two-types approach.
    /// `last` is the most recently seen type, `second` is the other type in
    /// the window. `last_count` tracks the contiguous run length of `last`
    /// ending at the current position. When a third type appears, the window
    /// resets to `last_count + 1` (the run of `last` plus the new fruit).
    pub fn total_fruit2(fruits: Vec<i32>) -> i32 {
        let mut last = -1;
        let mut second = -1;
        let mut last_count: i32 = 0;
        let mut cur: i32 = 0;
        let mut ans: i32 = 0;

        for &f in &fruits {
            if f == last || f == second {
                cur += 1;
            } else {
                cur = last_count + 1;
            }

            if f == last {
                last_count += 1;
            } else {
                last_count = 1;
                second = last;
                last = f;
            }

            ans = ans.max(cur);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 1]), 3);
        assert_eq!(Solution::total_fruit2(vec![1, 2, 1]), 3);
    }

    #[test]
    fn test_three_types() {
        assert_eq!(Solution::total_fruit(vec![0, 1, 2, 2]), 3);
        assert_eq!(Solution::total_fruit2(vec![0, 1, 2, 2]), 3);
    }

    #[test]
    fn test_pick_longest() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 3, 2, 2]), 4);
        assert_eq!(Solution::total_fruit2(vec![1, 2, 3, 2, 2]), 4);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::total_fruit(vec![1]), 1);
        assert_eq!(Solution::total_fruit2(vec![1]), 1);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::total_fruit(vec![5, 5, 5, 5]), 4);
        assert_eq!(Solution::total_fruit2(vec![5, 5, 5, 5]), 4);
    }

    #[test]
    fn test_two_types() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 1, 2, 1]), 5);
        assert_eq!(Solution::total_fruit2(vec![1, 2, 1, 2, 1]), 5);
    }

    #[test]
    fn test_alternating_three() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 3, 1, 2, 3]), 2);
        assert_eq!(Solution::total_fruit2(vec![1, 2, 3, 1, 2, 3]), 2);
    }

    #[test]
    fn test_long_run_then_switch() {
        assert_eq!(Solution::total_fruit(vec![1, 1, 1, 1, 2, 3, 3, 3]), 5);
        assert_eq!(Solution::total_fruit2(vec![1, 1, 1, 1, 2, 3, 3, 3]), 5);
    }
}
