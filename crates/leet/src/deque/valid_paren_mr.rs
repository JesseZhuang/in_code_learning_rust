// lc 1249

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let ss = &s;
        let mut res = Vec::new();
        let (mut l, mut r) = (0, 0);
        for c in ss.bytes() { if c == b')' { r += 1; } }
        for c in ss.bytes() {
            if c == b'(' {
                if l == r { continue; } else { l += 1; }
            } else if c == b')' {
                r -= 1;
                if l == 0 { continue; }
                l -= 1;
            }
            res.push(c);
        }
        String::from_utf8(res).expect("invalid utf8")
    }
}

struct Solution;