use std::collections::HashSet;

pub struct Solution;

impl Solution {
    // 0ms 2.17mb
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let mut res = String::with_capacity(n);
        for i in 0..n {
            let c = nums[i].as_bytes()[i] as char;
            res.push(if c == '0' { '1' } else { '0' });
        }
        res
    }

    // 0ms 2.24mb
    pub fn find_different_binary_string_set(nums: Vec<String>) -> String {
        let n = nums.len();
        let set: HashSet<String> = nums.iter().cloned().collect();
        let total = 1usize << n;
        for mask in 0..total {
            let mut candidate = String::with_capacity(n);
            for i in (0..n).rev() {
                let bit = (mask >> i) & 1;
                candidate.push(if bit == 1 { '1' } else { '0' });
            }
            if !set.contains(&candidate) {
                return candidate;
            }
        }
        unreachable!("All possible strings are present")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_valid(nums: Vec<String>, solver: fn(Vec<String>) -> String) {
        let ans = solver(nums.clone());
        assert_eq!(ans.len(), nums.len());
        assert!(!nums.contains(&ans));
    }

    fn assert_valid_both(nums: Vec<String>) {
        assert_valid(nums.clone(), Solution::find_different_binary_string);
        assert_valid(nums, Solution::find_different_binary_string_set);
    }

    #[test]
    fn example_1() {
        let nums = vec!["01".to_string(), "10".to_string()];
        assert_valid_both(nums);
    }

    #[test]
    fn example_2() {
        let nums = vec!["00".to_string(), "01".to_string()];
        assert_valid_both(nums);
    }

    #[test]
    fn example_3() {
        let nums = vec!["10".to_string(), "11".to_string()];
        assert_valid_both(nums);
    }

    #[test]
    fn edge_n1_zero() {
        let nums = vec!["0".to_string()];
        assert_valid_both(nums);
    }

    #[test]
    fn edge_n1_one() {
        let nums = vec!["1".to_string()];
        assert_valid_both(nums);
    }
}
