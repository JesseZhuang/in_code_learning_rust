/// tuple

#[cfg(test)]
mod tests {
    #[test]
    fn tuple_label_type() {
        let (i1, i2): (i32, i32) = (1, 2); // label outside, not inside
        assert_ne!(i1, i2);
    }
}