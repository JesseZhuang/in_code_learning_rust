pub struct Solution;

impl Solution {
    /// DFS iterative. O(N+E) time, O(N) space.
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        let mut visited = vec![false; n];
        let mut stack = vec![0usize];
        visited[0] = true;
        while let Some(room) = stack.pop() {
            for &key in &rooms[room] {
                let key = key as usize;
                if !visited[key] {
                    visited[key] = true;
                    stack.push(key);
                }
            }
        }
        visited.iter().all(|&v| v)
    }

    /// BFS. O(N+E) time, O(N) space.
    pub fn can_visit_all_rooms_bfs(rooms: Vec<Vec<i32>>) -> bool {
        use std::collections::VecDeque;
        let n = rooms.len();
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();
        visited[0] = true;
        queue.push_back(0usize);
        while let Some(room) = queue.pop_front() {
            for &key in &rooms[room] {
                let key = key as usize;
                if !visited[key] {
                    visited[key] = true;
                    queue.push_back(key);
                }
            }
        }
        visited.iter().all(|&v| v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // rooms = [[1],[2],[3],[]]
        let rooms = vec![vec![1], vec![2], vec![3], vec![]];
        assert!(Solution::can_visit_all_rooms(rooms.clone()));
        assert!(Solution::can_visit_all_rooms_bfs(rooms));
    }

    #[test]
    fn example2() {
        // rooms = [[1,3],[3,0,1],[2],[0]]
        let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];
        assert!(!Solution::can_visit_all_rooms(rooms.clone()));
        assert!(!Solution::can_visit_all_rooms_bfs(rooms));
    }

    #[test]
    fn single_room() {
        let rooms = vec![vec![]];
        assert!(Solution::can_visit_all_rooms(rooms.clone()));
        assert!(Solution::can_visit_all_rooms_bfs(rooms));
    }

    #[test]
    fn all_keys_in_first() {
        let rooms = vec![vec![1, 2, 3], vec![], vec![], vec![]];
        assert!(Solution::can_visit_all_rooms(rooms.clone()));
        assert!(Solution::can_visit_all_rooms_bfs(rooms));
    }

    #[test]
    fn chain() {
        // 0->1->2->3->4
        let rooms = vec![vec![1], vec![2], vec![3], vec![4], vec![]];
        assert!(Solution::can_visit_all_rooms(rooms.clone()));
        assert!(Solution::can_visit_all_rooms_bfs(rooms));
    }

    #[test]
    fn isolated() {
        // room 2 has no incoming key
        let rooms = vec![vec![1], vec![0], vec![]];
        assert!(!Solution::can_visit_all_rooms(rooms.clone()));
        assert!(!Solution::can_visit_all_rooms_bfs(rooms));
    }

    #[test]
    fn duplicates() {
        let rooms = vec![vec![1, 1, 1], vec![2, 2], vec![]];
        assert!(Solution::can_visit_all_rooms(rooms.clone()));
        assert!(Solution::can_visit_all_rooms_bfs(rooms));
    }

    #[test]
    fn cycle() {
        // 0->1->2->0, room 3 unreachable
        let rooms = vec![vec![1], vec![2], vec![0], vec![]];
        assert!(!Solution::can_visit_all_rooms(rooms.clone()));
        assert!(!Solution::can_visit_all_rooms_bfs(rooms));
    }
}
