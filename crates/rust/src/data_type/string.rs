#[cfg(test)]
mod tests {
    #[test]
    fn test_iterate() {
        let (s, v) = ("hello", vec!['h', 'e', 'l', 'l', 'o']);
        let mut i = 0;
        for c in s.chars() {
            assert_eq!(v[i], c);
            i += 1;
        }
        assert_eq!('h', s.chars().next().unwrap());
    }
}
