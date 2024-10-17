// lc 2530

use std::collections::BinaryHeap;

impl Solution {
    // credit @sunjesse
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut heap = BinaryHeap::from(nums);
        let mut res = 0;
        for _ in 0..k {
            let cur = heap.pop().unwrap();
            res += cur as i64;
            // f32 rounding error [756902131,995414896,95906472,149914376,387433380,848985151], k=6
            heap.push((cur + 2) / 3);
        }
        res
    }
}

struct Solution;