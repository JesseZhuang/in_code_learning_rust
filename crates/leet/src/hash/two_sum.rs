/// leet 1
/// companies: salesforce

use std::collections::HashMap;


impl Solution {
    pub fn two_sum_print(nums: Vec<i32>, target: i32) {
        let mut cnt = HashMap::new();
        for v in nums.iter() {
            let look = target - v;
            if let Some(c) = cnt.get(&look) {
                for _ in 0..*c { println!("{v} {look}") };
            }
            *cnt.entry(v).or_insert(0) += 1;
        }
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut val_ind = HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            let look = target - v;
            if let Some(&i1) = val_ind.get(&look) {
                return vec![i1 as i32, i as i32];
            }
            val_ind.insert(v, i);
        }
        unreachable!("should have found a pair");
    }
}

struct Solution;

#[cfg(test)]

mod tests {
    use crate::hash::two_sum::Solution;

    #[test]
    fn test_two_sum_print() {
        let v = vec![-2, -2, 2, -2, -2, 2, 3]; // 8 pairs
        Solution::two_sum_print(v, 0);
    }
}
