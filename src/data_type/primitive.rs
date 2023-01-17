/// primitive data types

use std::mem;
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

    let xs: [i32; 5] = [1, 2, 3, 4, 5]; // type signature is superfluous
    println!("array occupies {} bytes", mem::size_of_val(&xs)); // Arrays are stack allocated
    println!("address {:p}-{:p}", &xs[0], &xs[1]); // 0x7ff7b49b30bc-0x7ff7b49b30c0,4 bytes each
    let ys = [1; 5];
    println!("address {:p}", &ys[0]); // 0x7ff7b49b317c
    // todo: address going up, because not recursive?
}

#[allow(dead_code)]
fn reverse_tuple(pair: (i32, i32)) -> (i32, i32) {
    let (v1, v2) = pair; // unpack, destructure to create bindings
    (v2, v1)
}

pub fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

#[cfg(test)]
mod tests {
    use std::mem::MaybeUninit;

    use crate::data_type::primitive::{reverse_tuple, type_of};

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
        println!("{:?}", tuple_of_tuples); //((1, 2, 2), (4, -1), -2)
        assert_eq!(1, tuple_of_tuples.0.0);
        assert_eq!(1, (1u32, ).0);
        assert_eq!(1, (1u32)); // integer, not tuple
        assert_eq!((1, 2, ), reverse_tuple((2, 1)));
    }

    #[test]
    fn test_array_init() {
        let arr = [1, 2, 3];
        assert_eq!("[i32; 3]", type_of(&arr));
        let arr2 = [1; 10];
        for n in arr2 {
            assert_eq!(1, n);
        }
        assert_eq!(10, arr2.len());

        // declare array without init
        // probably make more sense to just use vec or initialize with some default value 0 or -1
        // can checkout crate array_init
        let mut arr2: [MaybeUninit<i32>; 5] = unsafe { MaybeUninit::uninit().assume_init() };
        arr2[2].write(3);
        unsafe { assert_eq!(3, arr2[2].assume_init()) };
        unsafe {
            for elem in arr2 {
                println!("{}", elem.assume_init()); // 0,0,3,0,39927808
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_array_safe_access() {
        let arr = [1, 2];
        for i in 0..arr.len() + 1 {
            match arr.get(i) {
                Some(val) => assert_eq!(*val, arr[i]),
                None => println!("out of bound index: {i}")
            }
        }
        arr.get(2).expect("should panic");
    }

    #[test]
    fn test_slice() {
        let arr = [1, 2, 3, 4, 5];
        let slice = &arr[1..3];  // [2,3]
        assert_eq!(2, slice.len());
        assert_eq!(3, slice[1]);
    }
}