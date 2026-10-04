/// LeetCode 2602 - Minimum Operations to Make All Array Elements Equal
pub struct Solution;

impl Solution {
    /// Sorts the values and uses prefix sums to calculate each query in O(log n) time.
    /// Overall time is O((n + m) log n) and extra space is O(n).
    pub fn min_operations(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
        nums.sort_unstable();

        let mut prefix_sums = Vec::with_capacity(nums.len() + 1);
        prefix_sums.push(0_i64);
        for &value in &nums {
            prefix_sums.push(prefix_sums.last().unwrap() + i64::from(value));
        }

        let total_count = nums.len() as i64;
        queries
            .into_iter()
            .map(|query| {
                let target = i64::from(query);
                let split_index = nums.partition_point(|&value| value < query);
                let left_count = split_index as i64;
                let left_cost = target * left_count - prefix_sums[split_index];

                let right_count = total_count - left_count;
                let right_sum = prefix_sums[nums.len()] - prefix_sums[split_index];
                let right_cost = right_sum - target * right_count;

                left_cost + right_cost
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_the_examples() {
        assert_eq!(
            Solution::min_operations(vec![3, 1, 6, 8], vec![1, 5]),
            vec![14, 10]
        );
        assert_eq!(
            Solution::min_operations(vec![2, 9, 6, 3], vec![10]),
            vec![20]
        );
    }

    #[test]
    fn handles_queries_below_at_and_above_the_values() {
        assert_eq!(
            Solution::min_operations(vec![2, 5, 9], vec![1, 5, 12]),
            vec![13, 7, 20]
        );
    }

    #[test]
    fn preserves_query_order_with_duplicate_values() {
        assert_eq!(
            Solution::min_operations(vec![4, 1, 4, 1], vec![4, 1, 1]),
            vec![6, 6, 6]
        );
    }

    #[test]
    fn uses_64_bit_totals() {
        assert_eq!(
            Solution::min_operations(vec![1; 100_000], vec![1_000_000_000]),
            vec![99_999_999_900_000]
        );
    }
}
