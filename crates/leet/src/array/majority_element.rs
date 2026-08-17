/// LeetCode 169 - Majority Element

pub struct Solution;

impl Solution {
    /// Boyer-Moore Voting Algorithm — O(n) time, O(1) space
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut candidate, mut count) = (0, 0);
        for n in nums {
            if count == 0 {
                candidate = n;
            }
            count += if n == candidate { 1 } else { -1 };
        }
        candidate
    }

    /// Sort then return middle element — O(n log n) time
    pub fn majority_element_sort(nums: &mut Vec<i32>) -> i32 {
        nums.sort();
        nums[nums.len() / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn single_element() {
        assert_eq!(Solution::majority_element(vec![1]), 1);
        assert_eq!(Solution::majority_element_sort(&mut vec![1]), 1);
    }

    #[test]
    fn example_3_2_3() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element_sort(&mut vec![3, 2, 3]), 3);
    }

    #[test]
    fn example_2_2_1_1_1_2_2() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
        assert_eq!(Solution::majority_element_sort(&mut vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn all_same() {
        assert_eq!(Solution::majority_element(vec![5, 5, 5, 5]), 5);
        assert_eq!(Solution::majority_element_sort(&mut vec![5, 5, 5, 5]), 5);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(Solution::majority_element(vec![-1, -1, 2]), -1);
        assert_eq!(Solution::majority_element_sort(&mut vec![-1, -1, 2]), -1);
    }

    #[test]
    fn majority_at_end() {
        assert_eq!(Solution::majority_element(vec![1, 2, 3, 3, 3, 3, 3]), 3);
        assert_eq!(Solution::majority_element_sort(&mut vec![1, 2, 3, 3, 3, 3, 3]), 3);
    }
}
