/// vector tests

#[cfg(test)]
mod tests {
    #[test]
    fn test_next_back() {
        let v = vec![1, 2, 3];
        let mut v_i = v.iter();
        assert_eq!(Some(&v[0]), v_i.next());
        assert_eq!(Some(&v[2]), v_i.next_back());
        assert_eq!(Some(&v[1]), v_i.next());
        assert_eq!(None, v_i.next_back());
        assert_eq!(None, v_i.next());
    }
}