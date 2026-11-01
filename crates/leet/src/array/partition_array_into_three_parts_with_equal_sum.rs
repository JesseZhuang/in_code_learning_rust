pub struct Solution;

impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let total: i32 = arr.iter().sum();
        if total % 3 != 0 {
            return false;
        }

        let target = total / 3;
        let mut part_sum = 0;
        let mut parts = 0;

        // Two linear passes take O(n) time and O(1) extra space.
        // Reserve the last element so the remaining third part is nonempty.
        for &value in arr.iter().take(arr.len() - 1) {
            part_sum += value;
            if part_sum == target {
                parts += 1;
                part_sum = 0;
                if parts == 2 {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn examples() {
        assert!(Solution::can_three_parts_equal_sum(vec![
            0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1
        ]));
        assert!(!Solution::can_three_parts_equal_sum(vec![
            0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1
        ]));
        assert!(Solution::can_three_parts_equal_sum(vec![
            3, 3, 6, 5, -2, 2, 5, 1, -9, 4
        ]));
    }

    #[test]
    fn all_zero_parts() {
        assert!(Solution::can_three_parts_equal_sum(vec![0, 0, 0]));
        assert!(Solution::can_three_parts_equal_sum(vec![0, 0, 0, 0]));
    }

    #[test]
    fn negative_target() {
        assert!(Solution::can_three_parts_equal_sum(vec![-1, -1, -1]));
        assert!(Solution::can_three_parts_equal_sum(vec![-3, 1, -2, -2]));
    }

    #[test]
    fn total_not_divisible_by_three() {
        assert!(!Solution::can_three_parts_equal_sum(vec![1, 1, 2]));
    }

    #[test]
    fn third_part_must_be_nonempty() {
        assert!(!Solution::can_three_parts_equal_sum(vec![1, -1, 0]));
    }

    #[test]
    fn third_part_can_have_multiple_elements() {
        assert!(Solution::can_three_parts_equal_sum(vec![1, 1, 2, -1]));
    }

    #[test]
    fn matching_total_does_not_guarantee_partitions() {
        assert!(!Solution::can_three_parts_equal_sum(vec![1, 2, 3]));
    }

    #[test]
    fn maximum_size_and_values() {
        assert!(Solution::can_three_parts_equal_sum(vec![10_000; 49_998]));
    }
}
