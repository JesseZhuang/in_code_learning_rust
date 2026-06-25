/// leet 547
pub struct Solution;

impl Solution {
    /// DFS iterative with stack. O(n^2) time, O(n) space.
    pub fn find_circle_num(is_connected: &Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut visited = vec![false; n]; // O(n) space
        let mut provinces = 0;
        let mut stack = Vec::new(); // O(n) space worst case

        for i in 0..n { // O(n) outer loop
            if visited[i] {
                continue;
            }
            provinces += 1;
            stack.push(i);
            while let Some(city) = stack.pop() { // DFS traversal
                if visited[city] {
                    continue;
                }
                visited[city] = true;
                for j in 0..n { // O(n) neighbor scan
                    if is_connected[city][j] == 1 && !visited[j] {
                        stack.push(j);
                    }
                }
            }
        }
        provinces
    }

    /// Union Find with path compression and union by rank.
    /// O(n^2 * alpha(n)) time, O(n) space.
    pub fn find_circle_num_uf(is_connected: &Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut parent: Vec<usize> = (0..n).collect(); // O(n) space
        let mut rank = vec![0u32; n]; // O(n) space
        let mut provinces = n as i32;

        for i in 0..n { // O(n^2) iteration over upper triangle
            for j in (i + 1)..n {
                if is_connected[i][j] == 1 {
                    if Self::union(&mut parent, &mut rank, i, j) {
                        provinces -= 1; // merged two components
                    }
                }
            }
        }
        provinces
    }

    /// Find with path compression. Amortized O(alpha(n)).
    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = Self::find(parent, parent[x]); // path compression
        }
        parent[x]
    }

    /// Union by rank. Returns true if two distinct components were merged.
    fn union(parent: &mut Vec<usize>, rank: &mut Vec<u32>, x: usize, y: usize) -> bool {
        let rx = Self::find(parent, x);
        let ry = Self::find(parent, y);
        if rx == ry {
            return false; // already in same component
        }
        match rank[rx].cmp(&rank[ry]) { // union by rank
            std::cmp::Ordering::Less => parent[rx] = ry,
            std::cmp::Ordering::Greater => parent[ry] = rx,
            std::cmp::Ordering::Equal => {
                parent[ry] = rx;
                rank[rx] += 1;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn run(matrix: &[&[i32]], expected: i32) {
        let grid: Vec<Vec<i32>> = matrix.iter().map(|r| r.to_vec()).collect();
        assert_eq!(Solution::find_circle_num(&grid), expected);
        assert_eq!(Solution::find_circle_num_uf(&grid), expected);
    }

    #[test]
    fn example1_two_provinces() {
        // [[1,1,0],[1,1,0],[0,0,1]] -> 2
        run(&[&[1, 1, 0], &[1, 1, 0], &[0, 0, 1]], 2);
    }

    #[test]
    fn example2_three_provinces() {
        // [[1,0,0],[0,1,0],[0,0,1]] -> 3
        run(&[&[1, 0, 0], &[0, 1, 0], &[0, 0, 1]], 3);
    }

    #[test]
    fn all_connected() {
        run(&[&[1, 1, 1], &[1, 1, 1], &[1, 1, 1]], 1);
    }

    #[test]
    fn single_city() {
        run(&[&[1]], 1);
    }

    #[test]
    fn chain() {
        // 0-1-2-3 connected in a chain
        run(
            &[
                &[1, 1, 0, 0],
                &[1, 1, 1, 0],
                &[0, 1, 1, 1],
                &[0, 0, 1, 1],
            ],
            1,
        );
    }

    #[test]
    fn two_pairs() {
        // (0,1) and (2,3) are two separate pairs
        run(
            &[
                &[1, 1, 0, 0],
                &[1, 1, 0, 0],
                &[0, 0, 1, 1],
                &[0, 0, 1, 1],
            ],
            2,
        );
    }
}
