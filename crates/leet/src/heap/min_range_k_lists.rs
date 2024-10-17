// lc 632

use std::cmp::{max, Reverse};
use std::collections::{BTreeSet, BinaryHeap};

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        // type t3 = (i32, usize, usize);
        let (mut pq, mut max_v) = (BinaryHeap::new(), i32::MIN);
        for (r, v) in nums.iter().enumerate() {
            max_v = max(max_v, v[0]);
            pq.push((Reverse(v[0]), r, 0usize));
        }
        let (mut left, mut right) = (-10i32.pow(5), 10i32.pow(5));
        while !pq.is_empty() {
            let (Reverse(mut v), r, mut c) = pq.pop().unwrap();
            if max_v - v < right - left { (left, right) = (v, max_v); }
            if c + 1 == nums[r].len() { break; }
            c += 1;
            v = nums[r][c];
            max_v = max(max_v, v);
            pq.push((Reverse(v), r, c));
        }
        vec![left, right]
    }
}

impl Solution2 {
    // @SamoylenkoDmitry
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut inds, mut res) = (vec![0; nums.len()], vec![0, i32::MAX]);
        let mut tree = BTreeSet::new();
        for r in 0..nums.len() { tree.insert((nums[r][0], r)); } // v, row
        while tree.len() == nums.len() {
            let (Some(&(b, _)), Some(&(a, r))) = (tree.last(), tree.first())
            else { break };
            if b - a < res[1] - res[0] {
                res[0] = a;
                res[1] = b
            }
            tree.pop_first();
            inds[r] += 1; // keep track of which col for r to look at
            if inds[r] < nums[r].len() { tree.insert((nums[r][inds[r]], r)); }
        };
        res
    }
}

struct Solution;
struct Solution2;