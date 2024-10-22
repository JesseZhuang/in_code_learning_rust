// lc 3307

impl Solution {
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let mut k = k;
        k -= 1; // cannot use ilog2, k maybe 1
        let (len, mut cnt) = (63 - k.leading_zeros() as usize, 0);
        for i in (0..len + 1).rev() {
            if (k >> i & 1) > 0 { cnt += operations[i]; }
        }
        (b'a' + (cnt % 26) as u8) as char
    }
}

struct Solution;