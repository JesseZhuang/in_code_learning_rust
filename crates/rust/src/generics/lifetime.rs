/// every reference has a lifetime, need to specify for function or struct using references

/// not changing the lifetimes of any values passed in or returned
/// just specifying that the borrow checker should reject those don’t adhere to these constraints.
#[allow(dead_code)]
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

/// instance cannot outlive the reference
#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

/// 1. the compiler assigns a lifetime parameter to each parameter that’s a reference
/// 2. if there is only one input, that lifetime is assigned to all output lifetime parameters
/// 3. if one is &self or &mut self, the lifetime of self is assigned to all output

#[cfg(test)]
mod tests {
    use crate::generics::lifetime::{longest, ImportantExcerpt};

    #[test]
    fn test_lifetime() {
        let string1 = String::from("long string is long");

        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            assert_eq!(string1, result); // auto deref slice, compare str and String
            // cannot move the println outside the inner scope, result still
            // borrowing string2 after it is dropped
            println!("The longest string is {result}");
        }
    }

    #[test]
    fn test_struct_lifetime() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        let i = ImportantExcerpt { // i has same lifetime as first_sentence
            part: first_sentence,
        };
        assert_eq!(i.part, novel.split('.').next().unwrap());
    }
}
