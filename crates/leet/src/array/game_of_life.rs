pub struct Solution;

impl Solution {
    /// LeetCode 289 — Game of Life.
    /// O(mn) time, O(1) space using in-place state encoding.
    ///
    /// Strategy:
    /// 1. Encode next state in the 2nd bit of each cell.
    ///    - Current state = cell & 1
    ///    - Next state will be stored in bit 1 (cell & 2)
    /// 2. For each cell, count live neighbors using `& 1`.
    ///    - A live cell with 2 or 3 neighbors survives.
    ///    - A dead cell with exactly 3 neighbors becomes alive.
    /// 3. Second pass: right-shift all cells to move next state into current.
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }

        let m = board.len() as i32;
        let n = board[0].len() as i32;

        for i in 0..m {
            for j in 0..n {
                let mut live = 0;
                for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let ni = i + di;
                        let nj = j + dj;
                        if ni >= 0 && ni < m && nj >= 0 && nj < n {
                            live += board[ni as usize][nj as usize] & 1;
                        }
                    }
                }
                let cur = board[i as usize][j as usize] & 1;
                // Cell lives in next generation if:
                // - currently alive and 2 or 3 neighbors, OR
                // - currently dead and exactly 3 neighbors
                if (cur == 1 && (live == 2 || live == 3)) || (cur == 0 && live == 3) {
                    board[i as usize][j as usize] |= 2;
                }
            }
        }

        // Second pass: shift to get next state
        for row in board.iter_mut() {
            for cell in row.iter_mut() {
                *cell >>= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut board = vec![
            vec![0, 1, 0],
            vec![0, 0, 1],
            vec![1, 1, 1],
            vec![0, 0, 0],
        ];
        Solution::game_of_life(&mut board);
        assert_eq!(
            board,
            vec![
                vec![0, 0, 0],
                vec![1, 0, 1],
                vec![0, 1, 1],
                vec![0, 1, 0],
            ]
        );
    }

    #[test]
    fn example2() {
        let mut board = vec![vec![1, 1], vec![1, 0]];
        Solution::game_of_life(&mut board);
        assert_eq!(board, vec![vec![1, 1], vec![1, 1]]);
    }

    #[test]
    fn single_cell_alive() {
        let mut board = vec![vec![1]];
        Solution::game_of_life(&mut board);
        assert_eq!(board, vec![vec![0]]); // dies of loneliness
    }

    #[test]
    fn single_cell_dead() {
        let mut board = vec![vec![0]];
        Solution::game_of_life(&mut board);
        assert_eq!(board, vec![vec![0]]);
    }

    #[test]
    fn all_alive_2x2() {
        // Each cell has 3 neighbors, all alive → all survive
        let mut board = vec![vec![1, 1], vec![1, 1]];
        Solution::game_of_life(&mut board);
        assert_eq!(board, vec![vec![1, 1], vec![1, 1]]);
    }

    #[test]
    fn blinker_oscillator() {
        // Horizontal blinker → vertical blinker
        let mut board = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];
        Solution::game_of_life(&mut board);
        assert_eq!(
            board,
            vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
            ]
        );
    }
}
