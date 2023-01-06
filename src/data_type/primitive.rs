/// primitive data types
use typename::TypeName;

pub fn primitives() {
    // vars can be type annotated.
    let logical: bool = true;
    println!("type of var logical {logical} is {}", logical.type_name_of());
    let a_float: f64 = 1.0;  // Regular annotation
    println!("type of var a_float {} is {}", a_float, a_float.type_name_of());
    let an_integer = 5i32; // Suffix annotation
    println!("type of var an_integer {} is {}", an_integer, an_integer.type_name_of());

    // Or a default will be used.
    let default_float = 3.0; // `f64`
    println!("type of var default_float {} is {}", default_float, default_float.type_name_of());
    let default_integer = 7;   // `i32`
    println!("type of var default_integer {} is {}", default_integer, default_integer.type_name_of());

    // A type can also be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred from another line, interesting
    println!("type of var inferred_type {} is {}", inferred_type, inferred_type.type_name_of());
    inferred_type = 4294967296i64;
    println!("type of var inferred_type {} is {}", inferred_type, inferred_type.type_name_of());

    // A mutable var's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    println!("type of var mutable {} is {}", mutable, mutable.type_name_of());
    mutable = 21;
    println!("type of var mutable {} is {}", mutable, mutable.type_name_of());

    // Error! The type of a var can't be changed.
    // mutable = true;

    // vars can be overwritten with shadowing.
    let mutable = true;
    println!("i32 type is {}", std::any::type_name::<i32>());
    println!("type of var mutable {} is {}", mutable, mutable.type_name_of());
}

fn reverse_tuple(pair: (i32, i32)) -> (i32, i32) {
    let (v1, v2) = pair; // unpack, destructure to create bindings
    (v2, v1)
}

#[cfg(test)]
mod tests {
    use crate::data_type::primitive::reverse_tuple;

    #[test]
    fn test_literals() {
        assert_eq!(3, 1u32 + 2);
        assert_eq!(-1, 1i32 - 2); // 1u32-2 would not compile, error saying overflow
        assert_eq!(0b0001, 0b0011 & 0b0101u32);
        println!("{:04b}", 2i32); // in intellij, registry, turn on org.rust.test.nocapture
        assert_eq!(100_0000, 1_000_000u32);
        assert_eq!(12, 0o10 + 4);
        println!("rust void? unit type: {:?}", ())
    }

    #[test]
    fn test_tuples() {
        let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
        println!("{:?}", tuple_of_tuples);
        assert_eq!(1, tuple_of_tuples.0.0);
        assert_eq!(1, (1u32, ).0);
        assert_eq!(1, (1u32));
        assert_eq!((1, 2, ), reverse_tuple((2, 1)));
    }
}