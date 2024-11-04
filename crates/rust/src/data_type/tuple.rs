/// tuple

#[cfg(test)]
mod tests {
    #[test]
    fn tuple_label_type() {
        let (i1, i2): (i32, i32) = (1, 2); // label outside, not inside
        assert_ne!(i1, i2);
    }

    #[allow(dead_code)]
    fn reverse_tuple(pair: (i32, i32)) -> (i32, i32) {
        let (v1, v2) = pair; // unpack, destructure to create bindings
        (v2, v1)
    }

    #[test]
    fn test_tuples() {
        let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
        println!("{:?}", tuple_of_tuples); //((1, 2, 2), (4, -1), -2)
        assert_eq!(1, tuple_of_tuples.0.0);
        assert_eq!(1, (1u32,).0);
        assert_eq!(1, (1u32)); // integer, not tuple
        assert_eq!((1, 2,), reverse_tuple((2, 1)));
    }
}