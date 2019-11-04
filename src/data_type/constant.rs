// Globals are declared outside all other scopes.
pub static LANGUAGE: &str = "Rust";
pub const THRESHOLD: i32 = 10;

/// - `static` constant is a possibly `mut`able variable with 'static lifetime.
/// The static lifetime is inferred and does not have to be specified.
/// Accessing or modifying a mutable static variable is unsafe.
/// - `const` is an unchangeable value (the common case).
pub fn constants() {
    fn is_big(n: i32) -> bool {
        // Access constant in some function
        n > THRESHOLD
    }

    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line
}
