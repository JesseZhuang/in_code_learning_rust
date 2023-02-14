fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
    // FIXME ^ Prefix with an underscore to suppress the warning
    // Please note that warnings may not be shown in a browser

    let mut _mutable_integer = 7i32;
    let mut i2 = 2;
    println!("before: {}, {}", _mutable_integer, i2);

    {
        let _mutable_integer = _mutable_integer; // shadow, make immutable, freezing
        let i2shadow = i2;
        // Error! `_mutable_integer` is frozen in this scope
        // _mutable_integer = 50;
        // FIXME ^ Comment out this line

        i2 = 30; // ok
        println!("i2 changed to: {}", i2);
        println!("i2 shadow not changed: {}", i2shadow);

        // immutable shadow goes out of scope
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
}
