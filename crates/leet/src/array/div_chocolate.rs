// lc 1231

pub struct Solution;

impl Solution {
    // @param sweetness: an integer array
    // @param k: an integer
    // @return:  return the maximum total sweetness of the piece
    pub fn maximize_sweetness(sweetness: Vec<i32>, k: i32) -> i32 {
        // write your code here
        let (mut l, mut r) = (0, 0);
        for s in &sweetness { r += s }
        while l < r {
            let m = l + (r - l) / 2;
            if Solution::pieces(&sweetness, m) < k + 1 { r = m } else { l = m + 1 }
        }
        l - 1
    }

    fn pieces(sw: &Vec<i32>, mid: i32) -> i32 {
        let (mut sum, mut cnt) = (0, 0);
        for s in sw {
            sum += s;
            if sum >= mid {
                sum = 0;
                cnt += 1;
            }
        }
        cnt
    }
}