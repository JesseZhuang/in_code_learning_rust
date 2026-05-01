/// leet 2563

pub struct Solution;

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort();
        Self::cnt_less(&nums, upper as i64 + 1) - Self::cnt_less(&nums, lower as i64)
    }

    /// Count pairs with sum < value using two pointers.
    fn cnt_less(nums: &[i32], value: i64) -> i64 {
        let (mut l, mut r) = (0i64, nums.len() as i64 - 1);
        let mut res: i64 = 0;
        while l < r {
            let sum = nums[l as usize] as i64 + nums[r as usize] as i64;
            if sum < value {
                res += r - l;
                l += 1;
            } else {
                r -= 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_count_fair_pairs() {
        assert_eq!(6, Solution::count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6));
        assert_eq!(1, Solution::count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11));
    }
}
