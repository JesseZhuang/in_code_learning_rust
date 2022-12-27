/// rust collections

fn main() {
    let mut v = vec![false; 5];
    for b in &v {
        assert!(!b, "{} was not true", b);
    }
    for (i, b) in v.iter_mut().enumerate() {
        if i == 3 {
            *b = !*b;
        }
    }
    assert!(v.get(3).expect("error getting 3rd element from vector"));
}
