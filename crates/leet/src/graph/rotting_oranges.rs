use std::collections::VecDeque;

/// leet 994, 0 ms, 2.25 mb
impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n, mut res, mut fresh, mut q) =
            (grid.len(), grid[0].len(), 0, 0, VecDeque::new());
        for i in 0..m {
            for j in 0..n {
                match grid[i][j] {
                    2 => q.push_back((i, j)),
                    1 => fresh += 1,
                    _ => {}
                }
            }
        }
        while !q.is_empty() && fresh > 0 {
            for _ in 0..q.len() {
                let (x, y) = q.pop_front().expect("not empty");
                for (dx, dy) in vec![(0, usize::MAX), (usize::MAX, 0), (0, 1), (1, 0)] {
                    let (nx, ny) = (x + dx, y + dy);
                    if nx > m - 1 || ny > n - 1 || grid[nx][ny] == 2 || grid[nx][ny] == 0 { continue; }
                    fresh -= 1;
                    grid[nx][ny] = 2;
                    q.push_back((nx, ny));
                }
            }
            res += 1;
        }
        if fresh == 0 { res } else { -1 }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    #[test]
    fn test_q_size() {
        let mut q = VecDeque::from(vec![1]);
        for _ in 0..q.len() {
            println!("{:#?}", q.pop_front());
            q.push_back(2);
            q.push_back(3);
        }
        // q.len() not evaluated dynamically, only once
    }
}
