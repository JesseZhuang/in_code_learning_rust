#[cfg(test)]
mod tests {
    #[test]
    fn test_mut_reference() {
        let mut a = 5;
        let b = &mut a; // must be from mut var
        *b += 6;
        assert_eq!(11, a);
    }
}