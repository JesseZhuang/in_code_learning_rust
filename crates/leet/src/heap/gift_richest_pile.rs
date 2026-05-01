/// leet 2558

use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut pq: BinaryHeap<i32> = gifts.into_iter().collect();
        for _ in 0..k {
            let cur = pq.pop().unwrap();
            pq.push((cur as f64).sqrt() as i32);
        }
        pq.into_iter().map(|x| x as i64).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_pick_gifts() {
        assert_eq!(29, Solution::pick_gifts(vec![25, 64, 9, 4, 100], 4));
        assert_eq!(4, Solution::pick_gifts(vec![1, 1, 1, 1], 4));
    }
}
