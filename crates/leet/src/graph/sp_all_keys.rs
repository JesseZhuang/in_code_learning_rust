// lc 864

use std::cmp::max;
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());
        // starting pos: x,y mk max key value 'a':1, 'b':2
        let (mut x, mut y, mut mk) = (0, 0, 0);
        for r in 0..n {
            for (c, ch) in (&grid[r]).as_bytes().iter().enumerate() {
                if *ch == b'@' { (x, y) = (r, c); }
                if ch.is_ascii_lowercase() { mk = max(mk, *ch - b'a' + 1); }
            }
        }
        let (mut q, mut vis) = (VecDeque::new(), HashSet::new());
        let start = (x, y, 0u8);
        q.push_back(start);
        vis.insert(start);
        // wrapping add usize::MAX, -1
        let dirs = vec![(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)];
        let mut res = 0;
        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                let (cr, cc, ck) = q.pop_front().unwrap();
                if ck == (1 << mk) - 1 { return res; }
                for d in dirs.iter() {
                    let (nr, nc, mut nk) =
                        (cr.wrapping_add(d.0), cc.wrapping_add(d.1), ck);
                    if nr > n - 1 || nc > m - 1 { continue; } // usize, no need check < 0
                    let ch = grid[nr].as_bytes()[nc];
                    if ch == b'#' { continue; }
                    if ch.is_ascii_uppercase() && (nk & (1 << (ch - b'A'))) == 0 { continue; }
                    if ch.is_ascii_lowercase() { nk |= 1 << (ch - b'a'); }
                    let state = (nr, nc, nk);
                    if vis.contains(&state) { continue; }
                    q.push_back(state);
                    vis.insert(state);
                }
            }
            res += 1;
        }
        -1
    }
}

struct Solution;
