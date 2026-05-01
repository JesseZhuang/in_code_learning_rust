/// LeetCode 1346

pub struct Solution;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut seen = std::collections::HashSet::new();
        for n in arr {
            if seen.contains(&(2 * n)) || (n % 2 == 0 && seen.contains(&(n / 2))) {
                return true;
            }
            seen.insert(n);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::hash::check_if_n_double_exist::Solution;

    #[test]
    fn test_check_if_exist() {
        assert!(Solution::check_if_exist(vec![10, 2, 5, 3]));
        assert!(!Solution::check_if_exist(vec![3, 1, 7, 11]));
    }
}
