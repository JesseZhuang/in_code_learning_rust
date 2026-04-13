/// LeetCode 1848 - Minimum Distance to the Target Element
/// Time: O(n)
/// Space: O(1)
struct Solution;

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        nums.iter()
            .enumerate()
            .filter(|&(_, &v)| v == target)
            .map(|(i, _)| (i as i32 - start).abs())
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::get_min_distance(vec![1, 2, 3, 4, 5], 5, 3));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::get_min_distance(vec![1], 1, 0));
    }

    #[test]
    fn example_3() {
        assert_eq!(
            0,
            Solution::get_min_distance(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1, 0)
        );
    }

    #[test]
    fn target_at_start() {
        assert_eq!(0, Solution::get_min_distance(vec![5, 1, 2, 3, 4], 5, 0));
    }

    #[test]
    fn target_at_end() {
        assert_eq!(4, Solution::get_min_distance(vec![1, 2, 3, 4, 5], 5, 0));
    }

    #[test]
    fn multiple_occurrences() {
        assert_eq!(1, Solution::get_min_distance(vec![1, 2, 3, 2, 1], 2, 2));
    }

    #[test]
    fn start_between_two_targets() {
        assert_eq!(2, Solution::get_min_distance(vec![3, 1, 1, 1, 3], 3, 2));
    }

    #[test]
    fn all_same() {
        assert_eq!(0, Solution::get_min_distance(vec![7, 7, 7], 7, 1));
    }
}
