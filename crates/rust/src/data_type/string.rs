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

    #[test]
    fn test_init() {
        let s_ref: &str = "hello_r"; // a reference to string, string slice, immutable
        assert_eq!("Hello_r", s_ref.replace("h", "H")); // generate new string
        let mut s_owned = String::from("hello_o"); // owns the data
        s_owned.push('.');
        s_owned.push_str(s_ref);
        assert_eq!("hello_o.hello_r", s_owned);
    }

    #[test]
    fn get_slice() {
        let slice = "hello";
        assert_eq!(String::from("hello"), slice);
        assert_eq!("hello", slice);
        assert_eq!("hel", &slice[0..3]);
        assert_eq!("h", &slice[0..1]); // to index a string slice
    }
}
