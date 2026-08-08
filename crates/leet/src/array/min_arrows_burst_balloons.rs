pub struct Solution;

impl Solution {
    /// Sort by end coordinate, greedily shoot at each balloon's rightmost endpoint.
    /// Time: O(n log n), Space: O(1)
    pub fn find_min_arrow_shots(points: &mut Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }
        // Sort by end coordinate
        points.sort_by_key(|p| p[1]);
        let mut arrows = 1;
        let mut arrow_pos = points[0][1];
        // O(n) scan through sorted intervals
        for i in 1..points.len() {
            // If current balloon starts after arrow position, need a new arrow
            if points[i][0] > arrow_pos {
                arrows += 1;
                arrow_pos = points[i][1];
            }
        }
        arrows
    }

    /// Sort by start coordinate, track the shrinking overlap region.
    /// Time: O(n log n), Space: O(1)
    pub fn find_min_arrow_shots2(points: &mut Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }
        // Sort by start coordinate
        points.sort_by_key(|p| p[0]);
        let mut arrows = 1;
        let mut overlap_end = points[0][1];
        // O(n) scan: maintain the common overlap region's end
        for i in 1..points.len() {
            if points[i][0] <= overlap_end {
                // Shrink the overlap region
                overlap_end = overlap_end.min(points[i][1]);
            } else {
                // No overlap, need a new arrow
                arrows += 1;
                overlap_end = points[i][1];
            }
        }
        arrows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        assert_eq!(Solution::find_min_arrow_shots(&mut points.clone()), 2);
        assert_eq!(Solution::find_min_arrow_shots2(&mut points), 2);
    }

    #[test]
    fn test_example2() {
        let mut points = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
        assert_eq!(Solution::find_min_arrow_shots(&mut points.clone()), 4);
        assert_eq!(Solution::find_min_arrow_shots2(&mut points), 4);
    }

    #[test]
    fn test_example3() {
        let mut points = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        assert_eq!(Solution::find_min_arrow_shots(&mut points.clone()), 2);
        assert_eq!(Solution::find_min_arrow_shots2(&mut points), 2);
    }

    #[test]
    fn test_single_balloon() {
        let mut points = vec![vec![5, 10]];
        assert_eq!(Solution::find_min_arrow_shots(&mut points.clone()), 1);
        assert_eq!(Solution::find_min_arrow_shots2(&mut points), 1);
    }

    #[test]
    fn test_all_overlapping() {
        let mut points = vec![vec![1, 10], vec![2, 9], vec![3, 8], vec![4, 7]];
        assert_eq!(Solution::find_min_arrow_shots(&mut points.clone()), 1);
        assert_eq!(Solution::find_min_arrow_shots2(&mut points), 1);
    }

    #[test]
    fn test_touching_edges() {
        let mut points = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(Solution::find_min_arrow_shots(&mut points.clone()), 2);
        assert_eq!(Solution::find_min_arrow_shots2(&mut points), 2);
    }

    #[test]
    fn test_i32_min_max_range() {
        // [MIN,MIN] and [MAX,MAX] don't overlap; [MIN,MAX] covers both but
        // after sorting the greedy picks arrow at MIN (covers first two) then MAX.
        let mut points = vec![
            vec![i32::MIN, i32::MIN],
            vec![i32::MAX, i32::MAX],
            vec![i32::MIN, i32::MAX],
        ];
        assert_eq!(Solution::find_min_arrow_shots(&mut points.clone()), 2);
        assert_eq!(Solution::find_min_arrow_shots2(&mut points), 2);
    }
}
