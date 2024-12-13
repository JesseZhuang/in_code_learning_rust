/// leet 1

use std::collections::HashMap;


impl Solution {
    pub fn two_sum_print(nums: Vec<i32>, target: i32) {
        let mut val_id = HashMap::new();
        for v in nums.iter() {
            let look = target - v;
            match val_id.get(&look) {
                Some(c) => {
                    for _ in 0..*c { println!("{} {}", v, look) }
                    *val_id.entry(v).or_insert(0) += 1;
                }
                None => *val_id.entry(v).or_insert(0) += 1,
            };
        }
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut val_id = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            match val_id.get(&(target - v)) {
                Some(i1) => return vec![*i1 as i32, i as i32],
                None => val_id.insert(v, i),
            };
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
