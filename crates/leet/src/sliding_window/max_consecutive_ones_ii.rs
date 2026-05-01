/// leet 487

pub struct Solution;

impl Solution {
    /// Sliding window, O(n) time, O(1) space.
    pub fn find_max_consecutive_ones(nums: &[i32]) -> i32 {
        let (mut l, mut r, mut k) = (0usize, 0usize, 1i32);
        let n = nums.len();
        while r < n {
            if nums[r] == 0 {
                k -= 1;
            }
            r += 1;
            if k < 0 {
                if nums[l] == 0 {
                    k += 1;
                }
                l += 1;
            }
        }
        (r - l) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::find_max_consecutive_ones(&[1, 0, 1, 1, 0]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::find_max_consecutive_ones(&[1, 0, 1, 1, 0, 1]), 4);
    }

    #[test]
    fn all_ones() {
        assert_eq!(Solution::find_max_consecutive_ones(&[1, 1, 1]), 3);
    }

    #[test]
    fn alternating() {
        assert_eq!(Solution::find_max_consecutive_ones(&[1, 0, 1, 0, 1]), 3);
    }

    #[test]
    fn leading_zeros() {
        assert_eq!(Solution::find_max_consecutive_ones(&[1, 0, 1, 1, 0, 0, 0, 0]), 4);
    }
}
