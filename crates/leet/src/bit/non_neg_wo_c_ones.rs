// lc 600

struct Solution;

impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        let mut f = [1; 32];
        // let msb = n.ilog2() as usize; // most significant bit, can only populate upto msb
        f[1] = 2;
        for i in 2..32 { f[i] = f[i - 1] + f[i - 2]; }
        let (mut res, mut prev) = (0, false);
        for k in (0..31).rev() {
            if n & (1 << k) != 0 {
                res += f[k];
                if prev { return res; } else { prev = true; }
            } else { prev = false; }
        }
        res + 1
    }
}