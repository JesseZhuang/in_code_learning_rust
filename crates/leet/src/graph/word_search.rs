pub struct Solution;

impl Solution {
    // O(mn * 4^l) time, O(l) recursion stack space.
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let (m, n) = (board.len(), board[0].len());
        let word: Vec<char> = word.chars().collect();
        for i in 0..m {
            for j in 0..n {
                if Self::dfs(&mut board, &word, 0, i as i32, j as i32) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(board: &mut Vec<Vec<char>>, word: &[char], idx: usize, i: i32, j: i32) -> bool {
        if idx == word.len() {
            return true;
        }
        if i < 0 || i >= board.len() as i32 || j < 0 || j >= board[0].len() as i32 {
            return false;
        }
        let (r, c) = (i as usize, j as usize);
        if board[r][c] != word[idx] {
            return false;
        }
        let tmp = board[r][c];
        board[r][c] = '#';
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dr, dc) in dirs {
            if Self::dfs(board, word, idx + 1, i + dr, j + dc) {
                return true;
            }
        }
        board[r][c] = tmp;
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(Solution::exist(board, "ABCCED".to_string()));
    }

    #[test]
    fn test_example2() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(Solution::exist(board, "SEE".to_string()));
    }

    #[test]
    fn test_example3() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(!Solution::exist(board, "ABCB".to_string()));
    }

    #[test]
    fn test_single_cell() {
        assert!(Solution::exist(vec![vec!['A']], "A".to_string()));
        assert!(!Solution::exist(vec![vec!['A']], "B".to_string()));
    }

    #[test]
    fn test_full_board() {
        let board = vec![vec!['A', 'B'], vec!['C', 'D']];
        assert!(Solution::exist(board, "ABDC".to_string()));
    }
}
