// lc 2530

use std::collections::BinaryHeap;

impl Solution {
    // credit @sunjesse
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut heap = BinaryHeap::from(nums);
        let mut res = 0;
        for _ in 0..k {
            let cur = heap.pop().unwrap();
            res += cur as i64;
            // f32 rounding error [756902131,995414896,95906472,149914376,387433380,848985151], k=6
            heap.push((cur + 2) / 3);
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_all_equal() {
        assert_eq!(Solution::max_kelements(vec![10; 5], 5), 50);
    }

    #[test]
    fn example_mixed_values() {
        assert_eq!(Solution::max_kelements(vec![1, 10, 3, 3, 3], 3), 17);
    }

    #[test]
    fn single_operation_chooses_maximum() {
        assert_eq!(Solution::max_kelements(vec![1, 10, 3, 3, 3], 1), 10);
    }

    #[test]
    fn single_element_three_reaches_fixed_point() {
        assert_eq!(Solution::max_kelements(vec![3], 3), 5);
    }

    #[test]
    fn single_element_four_rounds_up() {
        assert_eq!(Solution::max_kelements(vec![4], 3), 7);
    }

    #[test]
    fn single_element_five_rounds_up() {
        assert_eq!(Solution::max_kelements(vec![5], 3), 8);
    }

    #[test]
    fn single_element_six_uses_exact_division() {
        assert_eq!(Solution::max_kelements(vec![6], 3), 9);
    }

    #[test]
    fn one_is_fixed_point_at_maximum_operations() {
        assert_eq!(Solution::max_kelements(vec![1], 100_000), 100_000);
    }

    #[test]
    fn heap_reorders_after_replacement() {
        assert_eq!(Solution::max_kelements(vec![9, 8], 4), 23);
    }

    #[test]
    fn score_exceeds_i32_maximum() {
        assert_eq!(
            Solution::max_kelements(vec![1_000_000_000; 3], 3),
            3_000_000_000
        );
    }

    #[test]
    fn maximum_input_and_operation_counts() {
        assert_eq!(
            Solution::max_kelements(vec![1_000_000_000; 100_000], 100_000),
            100_000_000_000_000
        );
    }

    #[test]
    fn large_values_use_integer_ceiling() {
        assert_eq!(
            Solution::max_kelements(
                vec![
                    756_902_131,
                    995_414_896,
                    95_906_472,
                    149_914_376,
                    387_433_380,
                    848_985_151,
                ],
                6,
            ),
            3_603_535_575
        );
    }
}
