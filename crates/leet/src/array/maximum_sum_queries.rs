pub struct Solution;

impl Solution {
    /// Sweeps points and queries by descending x, with reversed compressed y
    /// coordinates turning a suffix maximum into a Fenwick prefix maximum.
    /// Time: O(n log n + q log q + (n + q) log n); space: O(n + q).
    pub fn maximum_sum_queries(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let mut coordinates = nums2.clone();
        coordinates.sort_unstable();
        coordinates.dedup();
        let m = coordinates.len();

        let mut points: Vec<(i32, i32)> = nums1.into_iter().zip(nums2).collect();
        points.sort_unstable_by_key(|point| std::cmp::Reverse(point.0));
        let mut query_order: Vec<usize> = (0..queries.len()).collect();
        query_order.sort_unstable_by_key(|&i| std::cmp::Reverse(queries[i][0]));

        let mut tree = vec![-1; m + 1];
        let mut answers = vec![-1; queries.len()];
        let mut next_point = 0;
        for query_index in query_order {
            let x = queries[query_index][0];
            let y = queries[query_index][1];
            while next_point < points.len() && points[next_point].0 >= x {
                let (a, b) = points[next_point];
                // Larger b values have smaller, one-based Fenwick indices.
                let mut index = m - coordinates.binary_search(&b).unwrap();
                while index <= m {
                    tree[index] = tree[index].max(a + b);
                    index += index & index.wrapping_neg();
                }
                next_point += 1;
            }

            // The reversed prefix includes exactly the coordinates >= y.
            let mut index = m - coordinates.partition_point(|&b| b < y);
            while index > 0 {
                answers[query_index] = answers[query_index].max(tree[index]);
                index &= index - 1;
            }
        }
        answers
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn official_example_one_preserves_query_order() {
        assert_eq!(
            Solution::maximum_sum_queries(
                vec![4, 3, 1, 2],
                vec![2, 4, 9, 5],
                vec![vec![4, 1], vec![1, 3], vec![2, 5]],
            ),
            vec![6, 10, 7]
        );
    }

    #[test]
    fn official_example_two_uses_the_best_qualifying_sum() {
        assert_eq!(
            Solution::maximum_sum_queries(
                vec![3, 2, 5],
                vec![2, 3, 4],
                vec![vec![4, 4], vec![3, 2], vec![1, 1]],
            ),
            vec![9, 9, 9]
        );
    }

    #[test]
    fn official_example_three_has_no_qualifying_point() {
        assert_eq!(
            Solution::maximum_sum_queries(vec![2, 1], vec![2, 3], vec![vec![3, 3]]),
            vec![-1]
        );
    }

    #[test]
    fn no_qualifying_x_or_y_returns_negative_one() {
        assert_eq!(
            Solution::maximum_sum_queries(
                vec![3, 5],
                vec![7, 2],
                vec![vec![6, 1], vec![1, 8], vec![4, 3], vec![3, 6]],
            ),
            vec![-1, -1, -1, 10]
        );
    }

    #[test]
    fn duplicate_coordinates_and_inclusive_boundaries_keep_the_maximum() {
        assert_eq!(
            Solution::maximum_sum_queries(
                vec![4, 7, 7, 2],
                vec![5, 5, 3, 9],
                vec![vec![7, 5], vec![7, 3], vec![4, 6], vec![2, 9], vec![1, 4]],
            ),
            vec![12, 12, -1, 11, 12]
        );
    }

    #[test]
    fn maximum_values_support_sums_up_to_two_billion() {
        assert_eq!(
            Solution::maximum_sum_queries(
                vec![1_000_000_000, 999_999_999],
                vec![1_000_000_000, 1_000_000_000],
                vec![vec![1_000_000_000, 1_000_000_000]],
            ),
            vec![2_000_000_000]
        );
    }
}
