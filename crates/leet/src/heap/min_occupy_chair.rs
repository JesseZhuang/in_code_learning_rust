// lc 1942

use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};

struct Solution;

impl Solution {
    // credit @discreaminant2809
    pub fn smallest_chair(mut times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let (ta, n) = (times[target_friend as usize][0], times.len());
        times.sort_by_key(|v| v[0]);
        // rust heap is max heap
        let mut available: BinaryHeap<Reverse<i32>> = (0..n as _).map(Reverse).collect();
        let mut occupy = BinaryHeap::new(); // (leave time, chair)
        let mut chair = n as i32 - 1;
        for t in times.iter() {
            let (arrive, leave) = (t[0], t[1]);
            while occupy.peek().is_some_and(|&(Reverse(leave), _)| leave <= arrive) {
                let (_, chair) = occupy.pop().unwrap();
                available.push(Reverse(chair));
            }
            chair = available.pop().unwrap().0; // get value from Reverse(chair)
            if arrive == ta { return chair; }
            occupy.push((Reverse(leave), chair));
        }
        chair
        // unreachable!()
    }
}

struct Solution2;

impl Solution2 {
    pub fn smallest_chair(mut times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let (ta, n) = (times[target_friend as usize][0], times.len());
        let mut free = BTreeSet::from_iter(0..n as i32);
        let mut occupy = BinaryHeap::new();
        times.sort_by_key(|v| v[0]);
        let mut chair = -1;
        for t in times.iter() {
            let (arrive, leave) = (t[0], t[1]);
            while occupy.peek().is_some_and(|&(Reverse(leave), _)| leave <= arrive) {
                let (_, chair) = occupy.pop().unwrap();
                free.insert(chair);
            }
            chair = *free.first().unwrap_or(&chair);
            free.remove(&chair);
            if arrive == ta { break; }
            occupy.push((Reverse(leave), chair));
        }
        chair
    }
}
