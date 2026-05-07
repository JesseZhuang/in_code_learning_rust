pub struct Solution;

impl Solution {
    // O(n) time, O(1) space. Two pointers.
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut right = height.len() - 1;
        let mut max_water = 0i32;

        while left < right {
            let w = (right - left) as i32;
            let h = height[left].min(height[right]);
            max_water = max_water.max(w * h);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_water
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }

    #[test]
    fn test_two_ones() {
        assert_eq!(1, Solution::max_area(vec![1, 1]));
    }

    #[test]
    fn test_symmetric_ends() {
        assert_eq!(16, Solution::max_area(vec![4, 3, 2, 1, 4]));
    }

    #[test]
    fn test_all_equal() {
        assert_eq!(20, Solution::max_area(vec![5, 5, 5, 5, 5]));
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(3, Solution::max_area(vec![3, 100]));
    }

    #[test]
    fn test_tall_ends() {
        assert_eq!(80, Solution::max_area(vec![10, 1, 1, 1, 1, 1, 1, 1, 10]));
    }
}
