use rand::Rng;

/// lc 169 easy

struct Solution;

impl Solution {
    pub fn majority_element_bmv(nums: Vec<i32>) -> i32 { // 3ms, 2.3Mb, boyer-moore voting
        let (mut res, mut count) = (0, 0);
        for n in nums {
            if count == 0 { res = n; }
            count += if res == n { 1 } else { -1 };
        }
        res
    }

    pub fn majority_element_rand(nums: Vec<i32>) -> i32 { // 3ms, 2.8Mb, random
        let mut rng = rand::thread_rng();
        loop {
            let res = nums[rng.gen_range(0..nums.len())];
            if nums.iter().filter(|&&n| n == res).count() > nums.len() / 2 { return res; };
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::array::major_element::Solution;

    #[test]
    fn test_1() { assert_eq!(1, Solution::majority_element_rand(vec![2, 1, 1, 4, 1])); }
}