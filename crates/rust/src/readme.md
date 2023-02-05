# Basics

## Naming

- https://rust-lang.github.io/api-guidelines/naming.html

## Documentation

- https://github.com/rust-lang/rfcs/blob/master/text/1574-more-api-documentation-conventions.md#appendix-a-full-conventions-text
- https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html

# Project Managing

- Packages: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module

## Packages and Crates

Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package. Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. Cargo passes the crate root files to rustc to build the library or binary.

A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.

## Modules

We can organize the functions into nested modules. Create a new library by running `cargo new --lib <module_name>`; then put the code in Listing 7-1 into src/lib.rs to define some modules and function signatures.
