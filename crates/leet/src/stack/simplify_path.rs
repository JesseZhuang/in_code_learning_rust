pub struct Solution;

impl Solution {
    /// Stack approach. O(n) time, O(n) space.
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = Vec::new(); // O(n) space
        for part in path.split('/') { // O(n) time
            match part {
                ".." => { stack.pop(); }
                "" | "." => {}
                _ => stack.push(part),
            }
        }
        format!("/{}", stack.join("/"))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn basic() { assert_eq!(Solution::simplify_path("/home/".into()), "/home"); }
    #[test]
    fn double_dot() { assert_eq!(Solution::simplify_path("/../".into()), "/"); }
    #[test]
    fn multiple_slashes() { assert_eq!(Solution::simplify_path("/home//foo/".into()), "/home/foo"); }
    #[test]
    fn complex() { assert_eq!(Solution::simplify_path("/a/./b/../../c/".into()), "/c"); }
    #[test]
    fn root() { assert_eq!(Solution::simplify_path("/".into()), "/"); }
    #[test]
    fn deep_path() { assert_eq!(Solution::simplify_path("/a/b/c/d".into()), "/a/b/c/d"); }
    #[test]
    fn triple_dots_file() { assert_eq!(Solution::simplify_path("/...".into()), "/..."); }
    #[test]
    fn double_dot_middle() { assert_eq!(Solution::simplify_path("/a/b/../c/d/../e".into()), "/a/c/e"); }
    #[test]
    fn pop_beyond_root() { assert_eq!(Solution::simplify_path("/a/../../b".into()), "/b"); }
}
