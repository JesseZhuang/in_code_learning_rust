pub struct Solution;

impl Solution {
    /// Bucket counts capped at n: O(n) time and O(n) extra space.
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut buckets = vec![0; n + 1];

        for citation in citations {
            buckets[(citation as usize).min(n)] += 1;
        }

        let mut papers = 0;
        for h in (0..=n).rev() {
            papers += buckets[h];
            if papers >= h {
                return h as i32;
            }
        }

        0
    }

    /// Sort a copy: O(n log n) time and O(n) extra space.
    pub fn h_index_sort(citations: Vec<i32>) -> i32 {
        let mut sorted = citations.clone();
        sorted.sort_unstable();

        let mut h_index = 0;
        for (index, &citation) in sorted.iter().rev().enumerate() {
            let papers = index + 1;
            if citation >= papers as i32 {
                h_index = papers as i32;
            } else {
                break;
            }
        }

        h_index
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(3, Solution::h_index(vec![3, 0, 6, 1, 5]));
        assert_eq!(3, Solution::h_index_sort(vec![3, 0, 6, 1, 5]));
    }

    #[test]
    fn example_two() {
        assert_eq!(1, Solution::h_index(vec![1, 3, 1]));
        assert_eq!(1, Solution::h_index_sort(vec![1, 3, 1]));
    }

    #[test]
    fn zero_citations() {
        assert_eq!(0, Solution::h_index(vec![0, 0, 0]));
        assert_eq!(0, Solution::h_index_sort(vec![0, 0, 0]));
    }

    #[test]
    fn all_papers_reach_n() {
        assert_eq!(4, Solution::h_index(vec![4, 5, 8, 10]));
        assert_eq!(4, Solution::h_index_sort(vec![4, 5, 8, 10]));
    }

    #[test]
    fn one_paper() {
        assert_eq!(1, Solution::h_index(vec![1]));
        assert_eq!(1, Solution::h_index_sort(vec![1]));
    }

    #[test]
    fn citations_above_n_are_capped() {
        assert_eq!(2, Solution::h_index(vec![100, 100]));
        assert_eq!(2, Solution::h_index_sort(vec![100, 100]));
    }
}
