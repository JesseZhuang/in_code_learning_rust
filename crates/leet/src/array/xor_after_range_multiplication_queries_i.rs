/// lc 3653

pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn xor_after_range_multiplication_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        for q in &queries {
            let (l, r, k, v) = (q[0] as usize, q[1] as usize, q[2] as usize, q[3] as i64);
            let mut idx = l;
            while idx <= r {
                nums[idx] = (nums[idx] as i64 * v % MOD) as i32;
                idx += k;
            }
        }
        nums.iter().fold(0, |acc, &x| acc ^ x)
    }
}

#[cfg(test)]
mod tests {
    use crate::array::xor_after_range_multiplication_queries_i::Solution;

    #[test]
    fn test_example1() {
        assert_eq!(
            4,
            Solution::xor_after_range_multiplication_queries(
                vec![1, 1, 1],
                vec![vec![0, 2, 1, 4]]
            )
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            31,
            Solution::xor_after_range_multiplication_queries(
                vec![2, 3, 1, 5, 4],
                vec![vec![1, 4, 2, 3], vec![0, 2, 1, 2]]
            )
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(
            6,
            Solution::xor_after_range_multiplication_queries(vec![3], vec![vec![0, 0, 1, 2]])
        );
    }

    #[test]
    fn test_large_values_mod() {
        let expected = (1_000_000_000i64 * 100_000 % 1_000_000_007) as i32;
        assert_eq!(
            expected,
            Solution::xor_after_range_multiplication_queries(
                vec![1_000_000_000],
                vec![vec![0, 0, 1, 100_000]]
            )
        );
    }
}
