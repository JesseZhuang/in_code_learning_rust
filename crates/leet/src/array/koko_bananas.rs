// leet 875, lint 1492, 4 ms, 2.44 mb
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (mut l, mut r) = (1, *piles.iter().max().expect("piles non-empty"));
        while l < r {
            let m = l + (r - l) / 2;
            if Self::feasible(&piles, h, m) { r = m } else { l = m + 1 }
        }
        l
    }
    fn feasible(piles: &Vec<i32>, h: i32, speed: i32) -> bool {
        let mut sum = 0;
        for p in piles {
            sum += (p - 1) / speed + 1;
        }
        sum <= h
    }
}

struct Solution;
