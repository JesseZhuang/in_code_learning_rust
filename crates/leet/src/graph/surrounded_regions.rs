use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    /// Approach 1: BFS from boundary
    /// Mark boundary-connected O's with '*', then flip remaining O→X, *→O.
    /// Time: O(mn), Space: O(mn)
    pub fn solve_bfs(board: &mut Vec<Vec<char>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }
        let m = board.len();
        let n = board[0].len();
        let mut queue = VecDeque::new();

        // Collect boundary O's
        for i in 0..m {
            for j in 0..n {
                if (i == 0 || i == m - 1 || j == 0 || j == n - 1) && board[i][j] == 'O' {
                    queue.push_back((i, j));
                    board[i][j] = '*';
                }
            }
        }

        // BFS to mark all boundary-connected O's
        let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in &dirs {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if board[nx][ny] == 'O' {
                        board[nx][ny] = '*';
                        queue.push_back((nx, ny));
                    }
                }
            }
        }

        // Flip: O→X (surrounded), *→O (not surrounded)
        for i in 0..m {
            for j in 0..n {
                match board[i][j] {
                    'O' => board[i][j] = 'X',
                    '*' => board[i][j] = 'O',
                    _ => {}
                }
            }
        }
    }

    /// Approach 2: Union-Find
    /// Union boundary O's with a dummy node, then flip O's not connected to dummy.
    /// Time: O(mn·α(mn)), Space: O(mn)
    pub fn solve_union_find(board: &mut Vec<Vec<char>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }
        let m = board.len();
        let n = board[0].len();
        let dummy = m * n; // virtual node for boundary-connected cells
        let mut uf = UnionFind::new(m * n + 1);

        for i in 0..m {
            for j in 0..n {
                if board[i][j] != 'O' {
                    continue;
                }
                let idx = i * n + j;
                if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                    uf.union(idx, dummy);
                }
                // Union with right and down neighbors
                if i + 1 < m && board[i + 1][j] == 'O' {
                    uf.union(idx, (i + 1) * n + j);
                }
                if j + 1 < n && board[i][j + 1] == 'O' {
                    uf.union(idx, i * n + j + 1);
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'O' && !uf.connected(i * n + j, dummy) {
                    board[i][j] = 'X';
                }
            }
        }
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
            }
        }
    }

    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_board(v: &[&[char]]) -> Vec<Vec<char>> {
        v.iter().map(|row| row.to_vec()).collect()
    }

    fn run_both(input: &[&[char]], expected: &[&[char]]) {
        let exp = to_board(expected);

        let mut board1 = to_board(input);
        Solution::solve_bfs(&mut board1);
        assert_eq!(board1, exp, "BFS failed");

        let mut board2 = to_board(input);
        Solution::solve_union_find(&mut board2);
        assert_eq!(board2, exp, "Union-Find failed");
    }

    #[test]
    fn test_example1() {
        // LeetCode example 1: 4x4 board
        let input: &[&[char]] = &[
            &['X', 'X', 'X', 'X'],
            &['X', 'O', 'O', 'X'],
            &['X', 'X', 'O', 'X'],
            &['X', 'O', 'X', 'X'],
        ];
        let expected: &[&[char]] = &[
            &['X', 'X', 'X', 'X'],
            &['X', 'X', 'X', 'X'],
            &['X', 'X', 'X', 'X'],
            &['X', 'O', 'X', 'X'],
        ];
        run_both(input, expected);
    }

    #[test]
    fn test_single_cell() {
        run_both(&[&['O']], &[&['O']]);
        run_both(&[&['X']], &[&['X']]);
    }

    #[test]
    fn test_all_o_border_2x2() {
        // All cells are on the border in a 2x2
        let input: &[&[char]] = &[&['O', 'O'], &['O', 'O']];
        let expected: &[&[char]] = &[&['O', 'O'], &['O', 'O']];
        run_both(input, expected);
    }

    #[test]
    fn test_inner_surrounded_3x3() {
        let input: &[&[char]] = &[
            &['X', 'X', 'X'],
            &['X', 'O', 'X'],
            &['X', 'X', 'X'],
        ];
        let expected: &[&[char]] = &[
            &['X', 'X', 'X'],
            &['X', 'X', 'X'],
            &['X', 'X', 'X'],
        ];
        run_both(input, expected);
    }

    #[test]
    fn test_single_row() {
        // All cells touch boundary in single row
        let input: &[&[char]] = &[&['O', 'X', 'O', 'X', 'O']];
        let expected: &[&[char]] = &[&['O', 'X', 'O', 'X', 'O']];
        run_both(input, expected);
    }

    #[test]
    fn test_connected_to_border() {
        // O's form a path from interior to border — should not be captured
        let input: &[&[char]] = &[
            &['X', 'X', 'X', 'X'],
            &['X', 'O', 'O', 'O'],
            &['X', 'O', 'X', 'X'],
            &['X', 'X', 'X', 'X'],
        ];
        let expected: &[&[char]] = &[
            &['X', 'X', 'X', 'X'],
            &['X', 'O', 'O', 'O'],
            &['X', 'O', 'X', 'X'],
            &['X', 'X', 'X', 'X'],
        ];
        run_both(input, expected);
    }
}
