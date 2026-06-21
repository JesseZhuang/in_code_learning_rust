use std::collections::HashSet;

pub struct Solution;

impl Solution {
    /// Time: O(1) — board is always 9x9, so at most 81 cells.
    /// Space: O(1) — HashSet holds at most 3*81 = 243 entries.
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut seen = HashSet::new();
        for i in 0..9 {
            for j in 0..9 {
                let c = board[i][j];
                if c == '.' {
                    continue;
                }
                let row = format!("r{}{}", i, c);
                let col = format!("c{}{}", j, c);
                let bx = format!("b{}{}{}", i / 3, j / 3, c);
                if !seen.insert(row) || !seen.insert(col) || !seen.insert(bx) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_board(raw: &[&str]) -> Vec<Vec<char>> {
        raw.iter().map(|row| row.chars().collect()).collect()
    }

    #[test]
    fn test_valid_board() {
        let board = to_board(&[
            "53..7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        assert!(Solution::is_valid_sudoku(board));
    }

    #[test]
    fn test_invalid_box() {
        // Place duplicate '5' in top-left 3x3 box
        let board = to_board(&[
            "53..7....",
            "6..195...",
            ".58....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        assert!(!Solution::is_valid_sudoku(board));
    }

    #[test]
    fn test_invalid_row() {
        let board = to_board(&[
            "53..7..5.",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        assert!(!Solution::is_valid_sudoku(board));
    }

    #[test]
    fn test_invalid_column() {
        let board = to_board(&[
            "53..7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "5...8..79",
        ]);
        assert!(!Solution::is_valid_sudoku(board));
    }

    #[test]
    fn test_empty_board() {
        let board = to_board(&[
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
        ]);
        assert!(Solution::is_valid_sudoku(board));
    }

    #[test]
    fn test_single_element() {
        let board = to_board(&[
            ".........",
            ".........",
            ".........",
            ".........",
            "....5....",
            ".........",
            ".........",
            ".........",
            ".........",
        ]);
        assert!(Solution::is_valid_sudoku(board));
    }
}
