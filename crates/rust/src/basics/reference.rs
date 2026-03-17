#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_mut_reference() {
        let mut a = 5;
        let b = &mut a; // must be from mut var
        *b += 6;
        assert_eq!(11, a);
    }

    #[test]
    fn test_mut_reference2() {
        let mut m = HashMap::new();
        m.insert(1, 3);
        let a = 1;
        assert_eq!(Some(&3), m.get(&a));
    }
}
